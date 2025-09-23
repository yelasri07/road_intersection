use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::{HEIGHT, WIDTH};

pub fn draw_roads(canvas: &mut Canvas<Window>) {
    let x = (WIDTH / 2) as i32;
    let y = (HEIGHT /2) as i32;

    canvas.set_draw_color(Color::WHITE);

    canvas.draw_line(Point::new(x, 0), Point::new(x, HEIGHT)).unwrap();
    canvas.draw_line(Point::new(x + 50, 0), Point::new(x + 50, HEIGHT)).unwrap();
    canvas.draw_line(Point::new(x - 50, 0), Point::new(x - 50, HEIGHT)).unwrap();

    canvas.draw_line(Point::new(0, y), Point::new(WIDTH, y)).unwrap();
    canvas.draw_line(Point::new(0, y + 50), Point::new(WIDTH, y + 50)).unwrap();
    canvas.draw_line(Point::new(0, y - 50), Point::new(WIDTH, y - 50)).unwrap();
}