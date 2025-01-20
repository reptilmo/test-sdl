extern crate sdl3;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::keyboard::Scancode;
use sdl3::pixels::Color;
use sdl3::rect::Point;
use sdl3::render::Canvas;
use sdl3::timer;
use sdl3::video::Window;

use std::collections::HashSet;
use std::time::Duration;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

fn draw_horizontal_line(canvas: &mut Canvas<Window>, x_min: i32, x_max: i32, y: i32) {
    for x in x_min..x_max {
        canvas.draw_point(Point::new(x, y)).unwrap();
    }
}

fn draw_vertical_line(canvas: &mut Canvas<Window>, x: i32, y_min: i32, y_max: i32) {
    for y in y_min..y_max {
        canvas.draw_point(Point::new(x, y)).unwrap();
    }
}

fn draw_box(canvas: &mut Canvas<Window>, upper_left: (i32, i32), lower_right: (i32, i32)) {
    let (x0, y0) = upper_left;
    let (x1, y1) = lower_right;

    for y in y0..y1 {
        draw_horizontal_line(canvas, x0, x1, y);
    }
}

fn draw_circle(canvas: &mut Canvas<Window>, origin: (i32, i32), radius: i32) {
    let (ox, oy) = origin;
    let r2 = radius * radius;

    for x in -radius..radius {
        let y: i32 = (f32::sqrt((r2 - x * x) as f32) + 0.5) as i32;
        draw_vertical_line(canvas, ox + x, oy - y, oy + y);
    }
}

struct Vec2 {
    x: f32,
    y: f32,
}

struct Ball {
    position: Vec2,
    direction: Vec2,
    velocity: f32,
    acceleration: f32,
}

fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("test-sdl", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut running = true;
    let mut current_ticks = timer::ticks();
    let mut new_ticks;
    let mut keys_pressed = HashSet::<Scancode>::new();
    let mut keys_were_pressed = HashSet::<Scancode>::new();

    let mut box_x: f32 = 399.0;
    let box_y: f32 = 749.0;

    let mut x0: f32;
    let mut x1: f32;
    let mut y0: f32;
    let mut y1: f32;

    let mut ball = Ball {
        position: Vec2 { x: 399.0, y: 399.0 },
        direction: Vec2 { x: 0.0, y: 1.0 },
        velocity: 0.5,
        acceleration: 0.0,
    };

    while running {
        new_ticks = timer::ticks();
        let ticks = new_ticks - current_ticks;
        current_ticks = new_ticks;
        //println!("Ticks: {}", ticks);

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

        keys_were_pressed = keys_pressed;
        keys_pressed = event_pump.keyboard_state().pressed_scancodes().collect();

        if keys_pressed.contains(&Scancode::Right) && keys_were_pressed.contains(&Scancode::Right) {
            box_x += 1.0 * ticks as f32;
        }

        if keys_pressed.contains(&Scancode::Left) && keys_were_pressed.contains(&Scancode::Left) {
            box_x -= 1.0 * ticks as f32;
        }

        x0 = box_x - 45.0;
        x1 = box_x + 45.0;
        y0 = box_y - 10.0;
        y1 = box_y + 10.0;

        if ball.position.x >= x0 && ball.position.x <= x1 && ball.position.y + 20.0 >= y0 {
            ball.position.y = y0 - 20.0;
            ball.direction.y = -1.0;
            ball.acceleration = 4.0;
        }

        if ball.position.y + 20.0 >= (HEIGHT - 10) as f32 {
            ball.position.y = (HEIGHT - 10) as f32 - 20.0;
            ball.direction.y = -ball.direction.y;
        } else if ball.position.y - 20.0 <= 10 as f32 {
            ball.position.y = 10 as f32 + 20.0;
            ball.direction.y = -ball.direction.y;
        }

        ball.position.x =
            ball.position.x + ball.direction.x * (ball.velocity + ball.acceleration) * ticks as f32;

        ball.position.y =
            ball.position.y + ball.direction.y * (ball.velocity + ball.acceleration) * ticks as f32;

        if ball.acceleration > 0.0 {
            ball.acceleration -= 0.01;
        } else {
            ball.acceleration = 0.0;
        }

        canvas.set_draw_color(Color::RGB(24, 24, 24));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 200, 0));
        draw_horizontal_line(&mut canvas, 10, WIDTH - 10, 10);
        draw_horizontal_line(&mut canvas, 10, WIDTH - 10, HEIGHT - 10);
        draw_vertical_line(&mut canvas, 10, 10, HEIGHT - 10);
        draw_vertical_line(&mut canvas, WIDTH - 10, 10, HEIGHT - 10);
        canvas.set_draw_color(Color::RGB(255, 64, 1));
        draw_circle(
            &mut canvas,
            (ball.position.x as i32, ball.position.y as i32),
            20,
        );

        canvas.set_draw_color(Color::RGB(1, 64, 255));
        draw_box(&mut canvas, (x0 as i32, y0 as i32), (x1 as i32, y1 as i32));

        canvas.present();
        std::thread::sleep(Duration::new(0, 5));
    }
}
