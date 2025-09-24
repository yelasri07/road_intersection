use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};
use std::time::{Duration, Instant};

use crate::roads::get_road_positions;
use crate::car::Direction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LightState {
    Red,
    Green,
}

pub struct TrafficLight {
    pub state: LightState,
    pub position: (i32, i32),
    pub direction: Direction,
    pub timer: Instant,
    pub duration: Duration,
    pub stop_line: (i32, i32), // Position where cars should stop
}

pub struct TrafficLightSystem {
    pub lights: Vec<TrafficLight>,
    pub current_light_index: usize,
    pub cycle_duration: Duration,
}

impl TrafficLightSystem {
    pub fn new() -> Self {
        let (x, y, width, height) = get_road_positions();
        
        // Create traffic lights for each approach to the intersection
        let mut lights = Vec::new();
        
        // North approach (coming from south, going north)
        lights.push(TrafficLight {
            state: LightState::Red,
            position: (x - 100, y - 150), // Moved further away from road
            direction: Direction::North,
            timer: Instant::now(),
            duration: Duration::from_secs(5),
            stop_line: (x - 50, y - 60), // Stop before intersection
        });
        
        // South approach (coming from north, going south)
        lights.push(TrafficLight {
            state: LightState::Red,
            position: (x + 50, y + 150), // Moved further away from road
            direction: Direction::South,
            timer: Instant::now(),
            duration: Duration::from_secs(5),
            stop_line: (x, y + 60), // Stop before intersection
        });
        
        // East approach (coming from west, going east)
        lights.push(TrafficLight {
            state: LightState::Red,
            position: (x - 150, y + 50), // Moved further away from road
            direction: Direction::East,
            timer: Instant::now(),
            duration: Duration::from_secs(5),
            stop_line: (x - 60, y), // Stop before intersection
        });
        
        // West approach (coming from east, going west)
        lights.push(TrafficLight {
            state: LightState::Red,
            position: (x + 150, y - 50), // Moved further away from road
            direction: Direction::West,
            timer: Instant::now(),
            duration: Duration::from_secs(5),
            stop_line: (x + 60, y - 50), // Stop before intersection
        });
        
        // Start with the first light as green
        if let Some(light) = lights.get_mut(0) {
            light.state = LightState::Green;
            light.timer = Instant::now();
        }
        
        TrafficLightSystem {
            lights,
            current_light_index: 0,
            cycle_duration: Duration::from_secs(5),
        }
    }
    
    pub fn update(&mut self) {
        let current_time = Instant::now();
        let current_light = &mut self.lights[self.current_light_index];
        
        // Check if it's time to switch to the next light
        if current_time.duration_since(current_light.timer) >= current_light.duration {
            // Turn current light red
            current_light.state = LightState::Red;
            current_light.timer = current_time;
            
            // Move to next light
            self.current_light_index = (self.current_light_index + 1) % self.lights.len();
            
            // Turn next light green
            let next_light = &mut self.lights[self.current_light_index];
            next_light.state = LightState::Green;
            next_light.timer = current_time;
        }
    }
    
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for light in &self.lights {
            let color = match light.state {
                LightState::Red => Color::RGB(255, 0, 0),
                LightState::Green => Color::RGB(0, 255, 0),
            };
            
            // Draw traffic light as a larger, more visible rectangle
            canvas.set_draw_color(color);
            canvas.fill_rect(Rect::new(light.position.0, light.position.1, 20, 20)).unwrap();
            
            // Draw stop line for debugging (optional)
            canvas.set_draw_color(Color::RGB(255, 255, 0)); // Yellow stop line
            match light.direction {
                Direction::North | Direction::South => {
                    canvas.draw_line(
                        sdl2::rect::Point::new(light.stop_line.0, light.stop_line.1),
                        sdl2::rect::Point::new(light.stop_line.0 + 50, light.stop_line.1)
                    ).unwrap();
                }
                Direction::East | Direction::West => {
                    canvas.draw_line(
                        sdl2::rect::Point::new(light.stop_line.0, light.stop_line.1),
                        sdl2::rect::Point::new(light.stop_line.0, light.stop_line.1 + 50)
                    ).unwrap();
                }
            }
        }
    }
    
    pub fn can_proceed(&self, direction: Direction) -> bool {
        for light in &self.lights {
            if light.direction == direction {
                return light.state == LightState::Green;
            }
        }
        false
    }
    
    // Get the stop line position for a given direction
    pub fn get_stop_line(&self, direction: Direction) -> Option<(i32, i32)> {
        for light in &self.lights {
            if light.direction == direction {
                return Some(light.stop_line);
            }
        }
        None
    }
    
    // Check if a car is approaching the stop line
    pub fn is_approaching_stop_line(&self, car_x: i32, car_y: i32, direction: Direction) -> bool {
        if let Some((stop_x, stop_y)) = self.get_stop_line(direction) {
            match direction {
                Direction::East => car_x <= stop_x && car_x + 50 >= stop_x - 30, // 30 pixels before stop line
                Direction::West => car_x >= stop_x && car_x <= stop_x + 30,
                Direction::North => car_y <= stop_y && car_y + 50 >= stop_y - 30,
                Direction::South => car_y >= stop_y && car_y <= stop_y + 30,
            }
        } else {
            false
        }
    }
    
    // Dynamic congestion control
    pub fn adjust_for_congestion(&mut self, cars: &[crate::car::Car], lane_capacity: usize) {
        let current_direction = self.lights[self.current_light_index].direction;
        
        let cars_in_lane = cars.iter()
            .filter(|car| car.direction == current_direction)
            .count();
        
        if cars_in_lane >= lane_capacity / 2 {
            self.lights[self.current_light_index].duration = Duration::from_secs(8);
        } else {
            self.lights[self.current_light_index].duration = Duration::from_secs(5);
        }
    }
}