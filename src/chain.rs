// chain.rs
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::util::{constrain_distance, constrain_angle, Vector2};

pub struct Chain {
    pub joints: Vec<Vector2>,
    link_size: f32,
    pub angles: Vec<f32>,
    angle_constraint: f32,
}

impl Chain {
    pub fn new(origin: Vector2, joint_count: usize, link_size: f32, angle_constraint: f32) -> Self {
        let mut joints = vec![origin];
        let mut angles = vec![0.0];
        for i in 1..joint_count {
            joints.push(Vector2::new(joints[i-1].x, joints[i-1].y + link_size));
            angles.push(0.0);
        }
        Chain {
            joints,
            link_size,
            angles,
            angle_constraint,
        }
    }

    pub fn resolve(&mut self, pos: Vector2) {
        self.angles[0] = (pos - self.joints[0]).heading();
        self.joints[0] = pos;
        for i in 1..self.joints.len() {
            let cur_angle = (self.joints[i-1] - self.joints[i]).heading();
            self.angles[i] = constrain_angle(cur_angle, self.angles[i-1], self.angle_constraint);
            self.joints[i] = self.joints[i-1] - Vector2::from_angle(self.angles[i]) * self.link_size;
        }
    }

    pub fn fabrik_resolve(&mut self, pos: Vector2, anchor: Vector2) {
        // Forward pass
        self.joints[0] = pos;
        for i in 1..self.joints.len() {
            self.joints[i] = constrain_distance(self.joints[i], self.joints[i-1], self.link_size);
        }

        // Backward pass
        *self.joints.last_mut().unwrap() = anchor;
        for i in (0..self.joints.len()-1).rev() {
            self.joints[i] = constrain_distance(self.joints[i], self.joints[i+1], self.link_size);
        }
    }

    pub fn display(&self, canvas: &mut Canvas<Window>) {
        for i in 0..self.joints.len() - 1 {
            let start = self.joints[i];
            let end = self.joints[i + 1];
            canvas.thick_line(start.x as i16, start.y as i16, end.x as i16, end.y as i16, 8, Color::RGB(255, 255, 255)).unwrap();
        }

        for joint in &self.joints {
            canvas.filled_circle(joint.x as i16, joint.y as i16, 16, Color::RGB(42, 44, 53)).unwrap();
        }
    }
}