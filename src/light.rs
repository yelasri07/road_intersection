use sdl2::pixels::Color;

pub struct Light {
    pub x: i32,
    pub y: i32,
    pub color: Color,
    pub status: bool,
}

impl Light {
    pub fn new(x: i32, y: i32, color: Color) -> Light {
        Light { x: x, y: y, color: color, status: false }
    }

    pub fn draw_traffic_light(){
        
    }
}