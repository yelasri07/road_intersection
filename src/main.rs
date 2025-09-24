use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::cell::Cell;
use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;

mod car;
mod light;
mod roads;

use crate::car::*;
use crate::light::*;
use crate::roads::*;
const WIDTH: i32 = 900;
const HEIGHT: i32 = 700;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("road_intersection", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let (x, y, _, _) = get_road_positions();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut cars: Vec<Car> = Vec::new();
    let mut lights: Vec<Light> = Vec::new();
    let mut capacity: HashMap<&str, u32> = HashMap::new();
    let timer = Instant::now();
    let mut a: u64 = 0;

    capacity.insert("North", 0);
    capacity.insert("South", 0);
    capacity.insert("East", 0);
    capacity.insert("West", 0);

    lights.push(Light::new(x - 100, y - 100, Color::RED, 1));
    lights.push(Light::new(x - 100, y + 50, Color::RED, 2));
    lights.push(Light::new(x + 50, y - 100, Color::RED, 3));
    lights.push(Light::new(x + 50, y + 50, Color::RED, 4));

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    let (x, y, width, height) = get_road_positions();

                    match key {
                        Keycode::Up => {
                            if let Some(new_car) = Car::new(x, height - 50, Direction::South, &cars)
                            {
                                cars.push(new_car);
                                if let Some(value) = capacity.get_mut("South") {
                                    *value += 1;
                                }
                            }
                        }
                        Keycode::Down => {
                            if let Some(new_car) = Car::new(x - 50, 0, Direction::North, &cars) {
                                cars.push(new_car);
                                if let Some(value) = capacity.get_mut("North") {
                                    *value += 1;
                                }
                            }
                        }
                        Keycode::Left => {
                            if let Some(new_car) =
                                Car::new(width - 50, y - 50, Direction::West, &cars)
                            {
                                cars.push(new_car);
                                if let Some(value) = capacity.get_mut("West") {
                                    *value += 1;
                                }
                            }
                        }
                        Keycode::Right => {
                            if let Some(new_car) = Car::new(0, y, Direction::East, &cars) {
                                cars.push(new_car);
                                if let Some(value) = capacity.get_mut("East") {
                                    *value += 1;
                                }
                            }
                        }
                        Keycode::R => {
                            if let Some(new_car) = Car::new_with_rand_dir(&cars, &mut capacity) {
                                cars.push(new_car);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        
        draw_roads(&mut canvas);

        if timer.elapsed().as_secs() >= a && Light::is_empty_center(&cars) {
            for l in lights.iter_mut() {
                l.draw_traffic_light(&mut canvas, &capacity, &cars);
            }
            
            a += 2;
        }
        
        for l in lights.iter_mut() {
            canvas.set_draw_color(l.color);
            canvas.fill_rect(Rect::new(l.x, l.y, 50, 50)).unwrap()
        }

        let copy_cars: Vec<Car> = cars.clone();
        for car in cars.iter_mut() {
            car.update_position(&mut lights, &copy_cars, &mut capacity);
            canvas.set_draw_color(car.color);
            canvas.fill_rect(car.rect()).unwrap();
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
