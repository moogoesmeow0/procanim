//lizard.rs
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::chain::Chain;
use crate::util::Vector2;

pub struct Lizard {
    spine: Chain,
    arms: Vec<Chain>,
    arm_desired: Vec<Vector2>,
    body_width: Vec<f32>,
}

impl Lizard {
    pub fn new(origin: Vector2) -> Self {
        let mut arms = Vec::new();
        let mut arm_desired = Vec::new();
        for i in 0..4 {
            arms.push(Chain::new(origin, 3, if i < 2 { 52.0 } else { 36.0 }, std::f32::consts::PI));
            arm_desired.push(Vector2::new(0.0, 0.0));
        }

        Lizard {
            spine: Chain::new(origin, 14, 64.0, std::f32::consts::PI / 8.0),
            arms,
            arm_desired,
            body_width: vec![52.0, 58.0, 40.0, 60.0, 68.0, 71.0, 65.0, 50.0, 28.0, 15.0, 11.0, 9.0, 7.0, 7.0],
        }
    }

    pub fn resolve(&mut self, mouse_x: f32, mouse_y: f32) {
        let head_pos = self.spine.joints[0];
        let mouse_pos = Vector2::new(mouse_x, mouse_y);
        let target_pos = head_pos + (mouse_pos - head_pos).set_mag(10.0);
        self.spine.resolve(target_pos);

        for i in 0..self.arms.len() {
            let side = if i % 2 == 0 { 1.0 } else { -1.0 };
            let body_index = if i < 2 { 3 } else { 7 };
            let angle = if i < 2 { std::f32::consts::PI / 4.0 } else { std::f32::consts::PI / 3.0 };
            let desired_pos = self.get_pos(body_index, angle * side, 80.0);

            if (desired_pos - self.arm_desired[i]).length() > 200.0 {
                self.arm_desired[i] = desired_pos;
            }

            let start = self.arms[i].joints[0];
            let end = self.get_pos(body_index, std::f32::consts::PI / 2.0 * side, -20.0);
            let target = start + (self.arm_desired[i] - start) * 0.4;
            self.arms[i].fabrik_resolve(target, end);
        }
    }

    pub fn display(&self, canvas: &mut Canvas<Window>) {
        self.draw_arms(canvas);
        self.draw_body(canvas);
        self.draw_eyes(canvas);
        //self.spine.display(canvas);
    }

    fn draw_arms(&self, canvas: &mut Canvas<Window>) {
        for (i, arm) in self.arms.iter().enumerate() {
            let shoulder = arm.joints[2];
            let foot = arm.joints[0];
            let mut elbow = arm.joints[1];

            let para = foot - shoulder;
            let perp = Vector2::new(-para.y, para.x).set_mag(30.0);

            if i == 2 {
                elbow = elbow - perp;
            } else if i == 3 {
                elbow = elbow + perp;
            }

            canvas.thick_line(shoulder.x as i16, shoulder.y as i16, elbow.x as i16, elbow.y as i16, 40, Color::RGB(255, 255, 255)).unwrap();
            canvas.thick_line(elbow.x as i16, elbow.y as i16, foot.x as i16, foot.y as i16, 40, Color::RGB(255, 255, 255)).unwrap();
            canvas.thick_line(shoulder.x as i16, shoulder.y as i16, elbow.x as i16, elbow.y as i16, 32, Color::RGB(82, 121, 111)).unwrap();
            canvas.thick_line(elbow.x as i16, elbow.y as i16, foot.x as i16, foot.y as i16, 32, Color::RGB(82, 121, 111)).unwrap();
        }
    }

    fn draw_body(&self, canvas: &mut Canvas<Window>) {
        let mut points: Vec<(i16, i16)> = Vec::new();

        // Right half of the lizard
        for i in 0..self.spine.joints.len() {
            let pos = self.get_pos(i, std::f32::consts::PI / 2.0, 0.0);
            points.push((pos.x as i16, pos.y as i16));
        }

        // Left half of the lizard
        for i in (0..self.spine.joints.len()).rev() {
            let pos = self.get_pos(i, -std::f32::consts::PI / 2.0, 0.0);
            points.push((pos.x as i16, pos.y as i16));
        }

        // Top of the head
        let head_top1 = self.get_pos(0, -std::f32::consts::PI / 6.0, -8.0);
        let head_top2 = self.get_pos(0, 0.0, -6.0);
        let head_top3 = self.get_pos(0, std::f32::consts::PI / 6.0, -8.0);
        points.push((head_top1.x as i16, head_top1.y as i16));
        points.push((head_top2.x as i16, head_top2.y as i16));
        points.push((head_top3.x as i16, head_top3.y as i16));

        canvas.filled_polygon(&points.iter().map(|&(x, y)| x).collect::<Vec<i16>>(), 
                      &points.iter().map(|&(x, y)| y).collect::<Vec<i16>>(), 
                      Color::RGB(172, 57, 49)).unwrap();
    }

    fn draw_eyes(&self, canvas: &mut Canvas<Window>) {
        let right_eye = self.get_pos(0, 3.0 * std::f32::consts::PI / 5.0, -7.0);
        let left_eye = self.get_pos(0, -3.0 * std::f32::consts::PI / 5.0, -7.0);
        canvas.filled_circle(right_eye.x as i16, right_eye.y as i16, 12, Color::RGB(255, 255, 255)).unwrap();
        canvas.filled_circle(left_eye.x as i16, left_eye.y as i16, 12, Color::RGB(255, 255, 255)).unwrap();
    }

    fn get_pos(&self, i: usize, angle_offset: f32, length_offset: f32) -> Vector2 {
        let joint = self.spine.joints[i];
        let angle = self.spine.angles[i];
        let width = self.body_width[i.min(self.body_width.len() - 1)];
        Vector2::new(
            joint.x + (angle + angle_offset).cos() * (width + length_offset),
            joint.y + (angle + angle_offset).sin() * (width + length_offset),
        )
    }
}
