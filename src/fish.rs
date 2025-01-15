//fish.rs
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::chain::Chain;
use crate::util::Vector2;

pub struct Fish {
    spine: Chain,
    body_color: Color,
    fin_color: Color,
    body_width: Vec<f32>,
}

impl Fish {
    pub fn new(origin: Vector2) -> Self {
        Fish {
            spine: Chain::new(origin, 12, 64.0, std::f32::consts::PI / 8.0),
            body_color: Color::RGB(58, 124, 165),
            fin_color: Color::RGB(129, 195, 215),
            body_width: vec![68.0, 81.0, 84.0, 83.0, 77.0, 64.0, 51.0, 38.0, 32.0, 19.0],
        }
    }

    pub fn resolve(&mut self, mouse_x: f32, mouse_y: f32) {
        let head_pos = self.spine.joints[0];
        let mouse_pos = Vector2::new(mouse_x, mouse_y);
        let target_pos = head_pos + (mouse_pos - head_pos).set_mag(16.0);
        self.spine.resolve(target_pos);
    }

    pub fn display(&self, canvas: &mut Canvas<Window>) {
        // Pectoral fins
        self.draw_fin(canvas, 3, std::f32::consts::PI / 3.0, 0.0, -std::f32::consts::PI / 4.0, 160.0, 64.0);
        self.draw_fin(canvas, 3, -std::f32::consts::PI / 3.0, 0.0, std::f32::consts::PI / 4.0, 160.0, 64.0);

        // Ventral fins
        self.draw_fin(canvas, 7, std::f32::consts::PI / 2.0, 0.0, -std::f32::consts::PI / 4.0, 96.0, 32.0);
        self.draw_fin(canvas, 7, -std::f32::consts::PI / 2.0, 0.0, std::f32::consts::PI / 4.0, 96.0, 32.0);

        // Caudal fin
        self.draw_caudal_fin(canvas);

        // Body
        self.draw_body(canvas);

        // Dorsal fin
        self.draw_dorsal_fin(canvas);

        // Eyes
        self.draw_eyes(canvas);
    }

    fn draw_fin(&self, canvas: &mut Canvas<Window>, i: usize, angle_offset: f32, length_offset: f32, rotation: f32, width: f32, height: f32) {
        let pos = self.get_pos(i, angle_offset, length_offset);
        canvas.filled_ellipse(pos.x as i16, pos.y as i16, (width / 2.0) as i16, (height / 2.0) as i16, self.fin_color).unwrap();
    }

    fn draw_caudal_fin(&self, canvas: &mut Canvas<Window>) {
        //caudal fin drawing
    }

    fn draw_body(&self, canvas: &mut Canvas<Window>) {
        //body drawing
    }

    fn draw_dorsal_fin(&self, canvas: &mut Canvas<Window>) {
        //dorsal fin drawing
    }

    fn draw_eyes(&self, canvas: &mut Canvas<Window>) {
        let right_eye = self.get_pos(0, std::f32::consts::PI / 2.0, -18.0);
        let left_eye = self.get_pos(0, -std::f32::consts::PI / 2.0, -18.0);
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
