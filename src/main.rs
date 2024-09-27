//main.rs
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod chain;
mod fish;
mod snake;
mod lizard;
mod util;

use fish::Fish;
use snake::Snake;
use lizard::Lizard;
use util::Vector2;

const SCREEN_WIDTH: u32 = 2400;
const SCREEN_HEIGHT: u32 = 1500;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Animal Simulation", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut fish = Fish::new(Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0));
    let mut snake = Snake::new(Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0));
    let mut lizard = Lizard::new(Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0));

    let mut animal = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown {..} => {
                    animal = (animal + 1) % 3;
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(40, 44, 52));
        canvas.clear();

        let mouse_state = event_pump.mouse_state();
        let (mouse_x, mouse_y) = (mouse_state.x() as f32, mouse_state.y() as f32);

        match animal {
            0 => {
                fish.resolve(mouse_x as f32, mouse_y as f32);
                fish.display(&mut canvas);
            },
            1 => {
                snake.resolve(mouse_x as f32, mouse_y as f32);
                snake.display(&mut canvas);
            },
            2 => {
                lizard.resolve(mouse_x as f32, mouse_y as f32);
                lizard.display(&mut canvas);
            },
            _ => unreachable!(),
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}