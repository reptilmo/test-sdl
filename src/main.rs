extern crate sdl3;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Point;
use sdl3::video::Window;
use sdl3::render::Canvas;
use std::time::Duration;

fn draw_box(
    canvas: &mut Canvas<Window>,
    upper_left: (i32, i32),
    lower_right: (i32, i32),
) {
    let (x0, y0) = upper_left;
    let (x1, y1) = lower_right;

    let mut draw_horizontal_line = |y: i32, x_min: i32, x_max: i32| {
        for x in x_min..x_max {
            canvas.draw_point(Point::new(x, y)).unwrap();
        }
    };

    for y in y0..y1 {
        draw_horizontal_line(y, x0, x1);
    }
}

fn draw_circle(
    canvas: &mut Canvas<Window>,
    origin: (i32, i32),
    radius: i32,
) {
    let (ox, oy) = origin;
    let r2 = radius * radius;

    let mut draw_vertical_line = |x: i32, y_min: i32, y_max: i32| {
        for y in y_min..y_max {
            canvas.draw_point(Point::new(x, y)).unwrap();
        }
    };

    for x in -radius..radius {
        let y: i32 = (f32::sqrt((r2 - x*x) as f32) + 0.5) as i32;
        draw_vertical_line(ox + x, oy - y, oy + y);
    }
}

fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("test-sdl", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut running = true;

    while running {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => running = false,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(1, 64, 255));
        draw_box(&mut canvas, (10, 10), (50, 50));
        canvas.set_draw_color(Color::RGB(255, 64, 1));
        draw_circle(&mut canvas, (399, 299), 20);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
