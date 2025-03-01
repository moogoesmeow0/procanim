//snake.rs
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::chain::Chain;
use crate::util::Vector2;

pub struct Snake {
    pub spine: Chain,
}

impl Snake {
    pub fn new(origin: Vector2) -> Self {
        Snake {
            spine: Chain::new(origin, 48, 64.0, std::f32::consts::PI / 8.0),
        }
    }

    pub fn resolve(&mut self, mouse_x: f32, mouse_y: f32) {
        let head_pos = self.spine.joints[0];
        let mouse_pos = Vector2::new(mouse_x, mouse_y);
        let target_pos = head_pos + (mouse_pos - head_pos).set_mag(8.0);
        self.spine.resolve(target_pos);
    }

    pub fn display(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(172, 57, 49));

        // Draw body
        let mut points: Vec<(f32, f32)> = Vec::new();
        
        // Right half of the snake
        for i in 0..self.spine.joints.len() {
            let pos = self.get_pos(i, std::f32::consts::PI / 2.0, 0.0);
            points.push((pos.x as f32, pos.y as f32));
        }

        // Left half of the snake
        for i in (0..self.spine.joints.len()).rev() {
            let pos = self.get_pos(i, -std::f32::consts::PI / 2.0, 0.0);
            points.push((pos.x as f32, pos.y as f32));
        }

        // Nose of snake
        let pos = self.get_pos(0_usize, 0.0, 0.0);
        points.push((pos.x as f32, pos.y as f32));

        crate::util::draw_spline_polygon(canvas, &points, 50, Color::RGB(172, 57, 49));

        
        // Draw eyes
        self.draw_eyes(canvas);
        //self.spine.display(canvas);
    }

    fn draw_eyes(&self, canvas: &mut Canvas<Window>) {
        let right_eye = self.get_pos(0, std::f32::consts::PI / 2.0, -18.0);
        let left_eye = self.get_pos(0, -std::f32::consts::PI / 2.0, -18.0);
        canvas.filled_circle(right_eye.x as i16, right_eye.y as i16, 12, Color::RGB(255, 255, 255)).unwrap();
        canvas.filled_circle(left_eye.x as i16, left_eye.y as i16, 12, Color::RGB(255, 255, 255)).unwrap();
    }

    fn body_width(&self, i: usize) -> f32 {
        match i {
            0 => 76.0,
            1 => 80.0,
            _ => 64.0 - i as f32,
        }
    }

    fn get_pos(&self, i: usize, angle_offset: f32, length_offset: f32) -> Vector2 {
        let joint = self.spine.joints[i];
        let angle = self.spine.angles[i];
        let width = self.body_width(i);
        Vector2::new(
            joint.x + (angle + angle_offset).cos() * (width + length_offset),
            joint.y + (angle + angle_offset).sin() * (width + length_offset),
        )
    }
}
