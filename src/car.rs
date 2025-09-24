use std::collections::HashMap;

use rand::Rng;
use sdl2::{pixels::Color, rect::Rect};

use crate::{light::Light, roads::get_road_positions};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Route {
    Left,
    Right,
    Straight,
}

#[derive(Debug, Clone, Copy)]
pub struct Car {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
    pub route: Route,
    pub color: Color,
    pub state: bool,
    pub out_calc: bool,
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
            id: existing_cars.len() as u32 + 1,
            x: x,
            y: y,
            direction: direction,
            route: route,
            color: color,
            state: true,
            out_calc: false,
        })
    }

    pub fn new_with_rand_dir(existing_cars: &[Car], capacity: &mut HashMap<&str, u32>) -> Option<Car> {
        let (x, y, width, height) = get_road_positions();

        let (spawn_x, spawn_y, direction, str_dir) = match rand::thread_rng().gen_range(1..=4) {
            1 => (0, y, Direction::East, "East"),
            2 => (x - 50, 0, Direction::North, "North"),
            3 => (x, height - 50, Direction::South, "South"),
            _ => (width - 50, y - 50, Direction::West, "West"),
        };

        if Car::is_position_safe(spawn_x, spawn_y, direction, existing_cars) {
            if let Some(value) = capacity.get_mut(str_dir) {
                *value += 1;
            } 
            return Car::new(spawn_x, spawn_y, direction, existing_cars);
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
                _ => {}
            }
        }

        true
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, 50, 50)
    }

    pub fn update_position(
        &mut self,
        lights: &mut [Light],
        cars: &[Car],
        capacity: &mut HashMap<&str, u32>,
    ) {

        let speed = 2;
        let (x, y, _, _) = get_road_positions();

        match self.direction {
            Direction::East => {
                if self.x > x - 100 {
                    self.state = false;
                    if !self.out_calc {
                        if let Some(value) = capacity.get_mut("East") {
                            if *value > 0 {
                                *value -= 1;
                            }
                            self.out_calc = true
                        }
                    }
                }

                if ((!lights[1].status && self.x == x - 100) || !self.is_can_move(cars))
                    && self.state
                {
                    return;
                }

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
                if self.x < x + 50 {
                    self.state = false;
                    if !self.out_calc {
                        if let Some(value) = capacity.get_mut("West") {
                            if *value > 0 {
                                *value -= 1;
                            }
                            self.out_calc = true
                        }
                    }
                }

                if ((!lights[2].status && self.x == x + 50) || !self.is_can_move(cars))
                    && self.state
                {
                    return;
                }

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
                if self.y > y - 100 {
                    if !self.out_calc {
                        if let Some(value) = capacity.get_mut("North") {
                            if *value > 0 {
                                *value -= 1;
                            }
                            self.out_calc = true
                        }
                    }
                }

                if ((!lights[0].status && self.y == y - 100) || !self.is_can_move(cars))
                    && self.state
                {
                    return;
                }

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
                if self.y < y + 50 {
                    self.state = false;
                    if !self.out_calc {
                        if let Some(value) = capacity.get_mut("South") {
                            if *value > 0 {
                                *value -= 1;
                            }
                            self.out_calc = true
                        }
                    }
                }

                if ((!lights[3].status && self.y == y + 50) || !self.is_can_move(cars))
                    && self.state
                {
                    return;
                }

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

    fn is_can_move(&self, cars: &[Car]) -> bool {
        let safety_distance = 70;

        for car in cars.iter() {
            match self.direction {
                Direction::East
                    if self.id > car.id
                        && car.direction == Direction::East
                        && self.x + safety_distance >= car.x =>
                {
                    return false;
                }

                Direction::West
                    if self.id > car.id
                        && car.direction == Direction::West
                        && self.x <= car.x + safety_distance =>
                {
                    return false;
                }

                Direction::North
                    if self.id > car.id
                        && car.direction == Direction::North
                        && self.y + safety_distance >= car.y =>
                {
                    return false;
                }

                Direction::South
                    if self.id > car.id
                        && car.direction == Direction::South
                        && self.y <= car.y + safety_distance =>
                {
                    return false;
                }

                _ => {}
            }
        }

        true
    }
}
