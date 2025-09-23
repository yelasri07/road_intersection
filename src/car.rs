use rand::Rng;
use sdl2::{pixels::Color};

pub enum Direction { North, South, East, West }
pub enum Route { Left, Right, Straight }

pub struct Car {
    x: i32,
    y: i32,
    direction: Direction,
    route: Route,
    color: Color,
    state: bool,
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
}