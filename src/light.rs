use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct Light {
    pub x: i32,
    pub y: i32,
    pub color: Color,
    pub status: bool,
    pub distance: i32,
}

impl Light {
    pub fn new(x: i32, y: i32, color: Color, distance: i32) -> Light {
        Light { x: x, y: y, color: color, status: false, distance: distance }
    }

    pub fn draw_traffic_light(&self, canvas: &mut Canvas<Window>){
        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(self.x, self.y,50, 50)).unwrap()
    }
}