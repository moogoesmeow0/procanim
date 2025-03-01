use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use rand;
use crate::util::Vector2;

const TARGET_DISTANCE: f32 = 30.0;

pub struct Toy {
    location: Vector2,
}

impl Toy{
    pub fn new(location: Vector2) -> Self{
        Toy {
            location
        }
    }

    pub fn update(&mut self, head: &Vector2){
        if ((head.x-self.location.x)*(head.x-self.location.x) + (head.y-self.location.y)*(head.y-self.location.y)).sqrt() <= TARGET_DISTANCE{
            self.location = Vector2::new(rand::random(), rand::random());
        }
    }

    pub fn display(&self, canvas: &mut Canvas<Window>){
        canvas.filled_circle(self.location.x as i16, self.location.y as i16, 12, Color::RGB(255, 255, 100)).unwrap();
    }
}
