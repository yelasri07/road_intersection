use rand::Rng;
use sdl2::{pixels::Color, rect::Rect};

use crate::roads::get_road_positions;
use crate::traffic_light::TrafficLightSystem;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Route {
    Left,
    Right,
    Straight,
}

#[derive(Debug, Clone)]
pub struct Car {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
    pub route: Route,
    pub color: Color,
    pub state: bool,
    pub speed: i32,
    pub stopped: bool,
}

impl Car {
    pub fn new(x: i32, y: i32, direction: Direction, existing_cars: &[Car]) -> Option<Car> {
        if !Car::is_position_safe(x, y, direction, existing_cars) {
            return None;
        }

        let rand_number = rand::thread_rng().gen_range(1..=3);
        let (route, color) = match rand_number {
            1 => (Route::Left, Color::RGB(0, 255, 128)),
            2 => (Route::Right, Color::RGB(255, 165, 0)),
            _ => (Route::Straight, Color::RGB(0, 191, 255)),
        };

        Some(Car {
            x: x,
            y: y,
            direction: direction,
            route: route,
            color: color,
            state: true,
            speed: 2,
            stopped: false,
        })
    }

    pub fn new_with_rand_dir(existing_cars: &[Car]) -> Option<Car> {
        let (x, y, width, height) = get_road_positions();

        let mut attempts = 0;
        while attempts < 10 {
            let (spawn_x, spawn_y, direction) = match rand::thread_rng().gen_range(1..=4) {
                1 => (0, y, Direction::East),
                2 => (x - 50, 0, Direction::North),
                3 => (x, height - 50, Direction::South),
                _ => (width - 50, y - 50, Direction::West),
            };

            if Car::is_position_safe(spawn_x, spawn_y, direction, existing_cars) {
                return Car::new(spawn_x, spawn_y, direction, existing_cars);
            }
            
            attempts += 1;
        }

        None
    }

    fn is_position_safe(x: i32, y: i32, direction: Direction, existing_cars: &[Car]) -> bool {
        let safety_distance = 70;
        
        for car in existing_cars {
            match direction {
                Direction::East if car.direction == Direction::East && car.y == y => {
                    if (car.x - x).abs() < safety_distance {
                        return false;
                    }
                }
                Direction::West if car.direction == Direction::West && car.y == y => {
                    if (car.x - x).abs() < safety_distance {
                        return false;
                    }
                }
                Direction::North if car.direction == Direction::North && car.x == x => {
                    if (car.y - y).abs() < safety_distance {
                        return false;
                    }
                }
                Direction::South if car.direction == Direction::South && car.x == x => {
                    if (car.y - y).abs() < safety_distance {
                        return false;
                    }
                }
                _ => {
                    let car_rect = Rect::new(car.x, car.y, 50, 50);
                    let new_car_rect = Rect::new(x, y, 50, 50);
                    if car_rect.has_intersection(new_car_rect) {
                        return false;
                    }
                    
                    let safety_zone = Car::get_safety_zone(x, y, direction, safety_distance);
                    if car_rect.has_intersection(safety_zone) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn get_safety_zone(x: i32, y: i32, direction: Direction, distance: i32) -> Rect {
        match direction {
            Direction::East => Rect::new(x + 50, y, distance as u32, 50),
            Direction::West => Rect::new(x - distance, y, distance as u32, 50),
            Direction::North => Rect::new(x, y + 50, 50, distance as u32),
            Direction::South => Rect::new(x, y - distance, 50, distance as u32),
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, 50, 50)
    }

    pub fn update_position(&mut self, traffic_lights: &TrafficLightSystem, other_car_positions: &[(i32, i32, Direction)]) {
        let (x, y, width, height) = get_road_positions();
        
        // Check if car should stop due to traffic light (only if approaching stop line)
        if traffic_lights.is_approaching_stop_line(self.x, self.y, self.direction) {
            if !traffic_lights.can_proceed(self.direction) {
                // Stop before the stop line
                if self.should_stop_at_light(traffic_lights) {
                    self.stopped = true;
                    return;
                }
            }
        }
        
        // Check if car should stop due to vehicle in front
        if self.should_stop_for_car(other_car_positions) {
            self.stopped = true;
            return;
        }
        
        self.stopped = false;
        
        // Move the car
        match self.direction {
            Direction::East => {
                self.x += self.speed;

                if self.route == Route::Left && self.x >= x {
                    self.direction = Direction::South;
                    self.route = Route::Straight;
                } else if self.route == Route::Right && self.x >= x - 50 {
                    self.direction = Direction::North;
                    self.route = Route::Straight;
                }
            }
            Direction::West => {
                self.x -= self.speed;
                if self.route == Route::Left && self.x + 50 <= x {
                    self.direction = Direction::North;
                    self.route = Route::Straight;
                } else if self.route == Route::Right && self.x <= x {
                    self.direction = Direction::South;
                    self.route = Route::Straight;
                }
            }
            Direction::North => {
                self.y += self.speed;
                if self.route == Route::Left && self.y >= y {
                    self.direction = Direction::East;
                    self.route = Route::Straight;
                } else if self.route == Route::Right && self.y >= y - 50 {
                    self.direction = Direction::West;
                    self.route = Route::Straight;
                }
            }
            Direction::South => {
                self.y -= self.speed;
                if self.route == Route::Left && self.y + 50 <= y {
                    self.direction = Direction::West;
                    self.route = Route::Straight;
                } else if self.route == Route::Right && self.y <= y {
                    self.direction = Direction::East;
                    self.route = Route::Straight;
                }
            }
        }
        
        // Remove car if it goes off screen
        if self.x < -100 || self.x > width + 100 || self.y < -100 || self.y > height + 100 {
            self.state = false;
        }
    }
    
    // Check if car should stop at traffic light
    fn should_stop_at_light(&self, traffic_lights: &TrafficLightSystem) -> bool {
        if let Some((stop_x, stop_y)) = traffic_lights.get_stop_line(self.direction) {
            match self.direction {
                Direction::East => self.x + 50 >= stop_x - 5, // Stop 5 pixels before line
                Direction::West => self.x <= stop_x + 5,
                Direction::North => self.y + 50 >= stop_y - 5,
                Direction::South => self.y <= stop_y + 5,
            }
        } else {
            false
        }
    }
    
    fn should_stop_for_car(&self, other_car_positions: &[(i32, i32, Direction)]) -> bool {
        let safety_distance = 30;
        
        for (other_x, other_y, other_direction) in other_car_positions {
            if *other_x == self.x && *other_y == self.y {
                continue;
            }
            
            match self.direction {
                Direction::East if *other_direction == Direction::East && *other_y == self.y => {
                    if other_x > &self.x && (other_x - self.x) < safety_distance + 50 {
                        return true;
                    }
                }
                Direction::West if *other_direction == Direction::West && *other_y == self.y => {
                    if other_x < &self.x && (self.x - other_x) < safety_distance + 50 {
                        return true;
                    }
                }
                Direction::North if *other_direction == Direction::North && *other_x == self.x => {
                    if other_y > &self.y && (other_y - self.y) < safety_distance + 50 {
                        return true;
                    }
                }
                Direction::South if *other_direction == Direction::South && *other_x == self.x => {
                    if other_y < &self.y && (self.y - other_y) < safety_distance + 50 {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
    
    pub fn get_position_data(&self) -> (i32, i32, Direction) {
        (self.x, self.y, self.direction)
    }
}