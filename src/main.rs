use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::gfx::framerate::{self, FPSManager};
use std::time::Duration;

mod chain;
mod fish;
mod snake;
mod lizard;
mod toy;
mod util;

use fish::Fish;
use snake::Snake;
use lizard::Lizard;
use util::Vector2;
use toy::Toy;

const SCREEN_WIDTH: u32 = 1720;
const SCREEN_HEIGHT: u32 = 1080;

const RATE: u32 = 60;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Animal Simulation", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut fps = FPSManager::new();

    let mut fish = Fish::new(Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0));
    let mut snake = Snake::new(Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0));
    let mut lizard = Lizard::new(Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0));
    let mut toy = Toy::new(Vector2::new(SCREEN_WIDTH as f32 / 3.0, SCREEN_HEIGHT as f32 / 3.0));

    let mut animal = 0;

    'running: loop {
        println!("{}", fps.get_framerate());
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

        toy.update(match animal {
            0 => &fish.spine.joints[0],
            1 => &snake.spine.joints[0],
            2 => &lizard.spine.joints[0],
            _ => unreachable!()
        });
        toy.display(&mut canvas);

        canvas.present();
        let _ = fps.set_framerate(RATE);
    }

    Ok(())
}
