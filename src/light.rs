use std::collections::HashMap;

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct Light {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub color: Color,
    pub status: bool,
}

impl Light {
    pub fn new(x: i32, y: i32, color: Color, id: u32) -> Light {
        Light { id: id ,x: x, y: y, color: color, status: false }
    }

    pub fn draw_traffic_light(&mut self, canvas: &mut Canvas<Window>, capacity: &HashMap<&str, u32>){
        let mut green_light: u32 = 1;
        let mut max_cars: u32 = 0;
        for (key, value) in capacity {
            if *value > max_cars {
                max_cars = *value;
                match *key {
                    "North" => green_light = 1,
                    "South" => green_light = 4,
                    "East" => green_light = 2,
                    _ => green_light = 3
                }
            }
        }

        if self.id == green_light {
            self.color = Color::GREEN;
            self.status = true
        } else {
            self.color = Color::RED;
            self.status = false
        }

        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(self.x, self.y,50, 50)).unwrap()
    }
}