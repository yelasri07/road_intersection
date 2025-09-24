use sdl2::{pixels::Color, rect::{Point}, render::Canvas, video::Window};

use crate::{HEIGHT, WIDTH};

pub fn draw_roads(canvas: &mut Canvas<Window>) {
    let (x, y, width, height) = get_road_positions();

   canvas.set_draw_color(Color::WHITE);

    canvas.draw_line(Point::new(x, 0), Point::new(x, height)).unwrap();
    canvas.draw_line(Point::new(x + 50, 0), Point::new(x + 50, height)).unwrap();
    canvas.draw_line(Point::new(x - 50, 0), Point::new(x - 50, height)).unwrap();

    canvas.draw_line(Point::new(0, y), Point::new(width, y)).unwrap();
    canvas.draw_line(Point::new(0, y + 50), Point::new(width, y + 50)).unwrap();
    canvas.draw_line(Point::new(0, y - 50), Point::new(width, y - 50)).unwrap();
}

pub fn get_road_positions() -> (i32, i32, i32, i32) {
    let x = (WIDTH / 2) as i32;
    let y = (HEIGHT /2) as i32;

    (x, y, WIDTH, HEIGHT)
}
