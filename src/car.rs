use rand::Rng;
use sdl2::{pixels::Color, rect::Rect};

use crate::roads::get_road_positions;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Route {
    Left,
    Right,
    Straight,
}

#[derive(Debug)]
pub struct Car {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
    pub route: Route,
    pub color: Color,
    pub state: bool,
}

impl Car {
    pub fn new(x: i32, y: i32, direction: Direction, existing_cars: &[Car]) -> Option<Car> {
        // Check if this position is safe from existing cars
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
        })
    }

    pub fn new_with_rand_dir(existing_cars: &[Car]) -> Option<Car> {
        let (x, y, width, height) = get_road_positions();
        // let safety_distance = 50; // Minimum distance from other cars

        // Try different spawn positions until we find a safe one
        let mut attempts = 0;
        while attempts < 10 { // Limit attempts to prevent infinite loop
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

        None // Could not find a safe position after several attempts
    }

    // Check if a position is safe from existing cars
    fn is_position_safe(x: i32, y: i32, direction: Direction, existing_cars: &[Car]) -> bool {
        let safety_distance = 70; // Minimum distance from other cars
        
        for car in existing_cars {
            match direction {
                Direction::East if car.direction == Direction::East && car.y == y => {
                    // Check if too close to cars coming from the same direction
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
                // Also check for cars in intersecting paths
                _ => {
                    let car_rect = Rect::new(car.x, car.y, 50, 50);
                    let new_car_rect = Rect::new(x, y, 50, 50);
                    if car_rect.has_intersection(new_car_rect) {
                        return false;
                    }
                    
                    // Check safety zone in front of the new car
                    let safety_zone = Car::get_safety_zone(x, y, direction, safety_distance);
                    if car_rect.has_intersection(safety_zone) {
                        return false;
                    }
                }
            }
        }
        true
    }

    // Get the safety zone in front of a car position
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

    pub fn update_position(&mut self) {
        let speed = 2;
        let (x, y, _, _) = get_road_positions();

        match self.direction {
            Direction::East => {
                self.x += speed;

                if self.route == Route::Left && self.x >= x {
                    self.direction = Direction::South;
                    self.route = Route::Straight;
                } else if self.route == Route::Right && self.x >= x - 50 {
                    self.direction = Direction::North;
                    self.route = Route::Straight;
                }
            }
            Direction::West => {
                self.x -= speed;
                if self.route == Route::Left && self.x + 50 <= x {
                    self.direction = Direction::North;
                    self.route = Route::Straight;
                } else if self.route == Route::Right && self.x <= x {
                    self.direction = Direction::South;
                    self.route = Route::Straight;
                }
            }
            Direction::North => {
                self.y += speed;
                if self.route == Route::Left && self.y >= y {
                    self.direction = Direction::East;
                    self.route = Route::Straight;
                } else if self.route == Route::Right && self.y >= y - 50 {
                    self.direction = Direction::West;
                    self.route = Route::Straight;
                }
            }
            Direction::South => {
                self.y -= speed;
                if self.route == Route::Left && self.y + 50 <= y {
                    self.direction = Direction::West;
                    self.route = Route::Straight;
                } else if self.route == Route::Right && self.y <= y {
                    self.direction = Direction::East;
                    self.route = Route::Straight;
                }
            }
        }
    }
}