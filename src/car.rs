use rand::Rng;
use sdl2::{pixels::Color, rect::Rect};

use crate::roads::get_road_positions;

#[derive(Debug, PartialEq, Eq)]

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
    pub fn new(x: i32, y: i32, direction: Direction) -> Car {
        let rand_number = rand::thread_rng().gen_range(1..=3);
        let (route, color) = match rand_number {
            1 => (Route::Left, Color::RGB(0, 255, 128)),
            2 => (Route::Right, Color::RGB(255, 165, 0)),
            _ => (Route::Straight, Color::RGB(0, 191, 255)),
        };

        Car {
            x: x,
            y: y,
            direction: direction,
            route: route,
            color: color,
            state: true,
        }
    }

    pub fn new_with_rand_dir() -> Car {
        let (x, y, width, height) = get_road_positions();

        let (x, y, direction) = match rand::thread_rng().gen_range(1..=4) {
            1 => (0, y, Direction::East),
            2 => (x - 50, 0, Direction::North),
            3 => (x, height - 50, Direction::South),
            _ => (width - 50, y - 50, Direction::West),
        };
        Car::new(x, y, direction)
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, 50, 50)
    }

    pub fn update_position(&mut self) {
        let (x, y, _, _) = get_road_positions();
        let speed = 2;

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
