use rand::Rng;
use sdl2::{pixels::Color, rect::Rect};

use crate::{HEIGHT, WIDTH};

pub enum Direction { North, South, East, West }
pub enum Route { Left, Right, Straight }

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
            _ => (Route::Straight, Color::RGB(0, 191, 255))
        };

        Car { x: x, y: y, direction: direction, route: route, color: color, state: true }
    }

    pub fn new_with_rand_dir() -> Car {
        let x = (WIDTH / 2) as i32;
        let y = (HEIGHT /2) as i32;

        let (x, y, direction) =    match rand::thread_rng().gen_range(1..=4) {
            1 => (0, y, Direction::East),
            2 => (x - 50, 0, Direction::North),
            3 => (x, HEIGHT - 50, Direction::South),
            _ => (WIDTH - 50, y - 50, Direction::West),
        };
        Car::new(x, y, direction)
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, 50, 50)
    }
}
