// util.rs
use std::f32::consts::PI;
use std::iter;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::gfx::primitives::DrawRenderer;


#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 { x, y }
    }

    pub fn from_angle(angle: f32) -> Self {
        Vector2 {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    pub fn heading(&self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn set_mag(&self, length: f32) -> Self {
        let mag = (self.x * self.x + self.y * self.y).sqrt();
        Vector2 {
            x: self.x * length / mag,
            y: self.y * length / mag,
        }
    }
    
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl std::ops::Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

pub fn constrain_distance(pos: Vector2, anchor: Vector2, constraint: f32) -> Vector2 {
    anchor + (pos - anchor).set_mag(constraint)
}

pub fn constrain_angle(angle: f32, anchor: f32, constraint: f32) -> f32 {
    let diff = relative_angle_diff(angle, anchor);
    if diff.abs() <= constraint {
        simplify_angle(angle)
    } else if diff > constraint {
        simplify_angle(anchor - constraint)
    } else {
        simplify_angle(anchor + constraint)
    }
}

pub fn relative_angle_diff(angle: f32, anchor: f32) -> f32 {
    let angle = simplify_angle(angle + PI - anchor);
    PI - angle
}

pub fn simplify_angle(mut angle: f32) -> f32 {
    while angle >= 2.0 * PI {
        angle -= 2.0 * PI;
    }
    while angle < 0.0 {
        angle += 2.0 * PI;
    }
    angle
}


fn quadratic_bezier_points(p0: (f32, f32), p1: (f32, f32), p2: (f32, f32), steps: usize) -> Vec<(i16, i16)> {
    let step = 1.0 / steps as f32;
    iter::successors(Some(0.0), |&t| Some(t + step).filter(|&t| t <= 1.0))
        .map(|t| {
            let u = 1.0 - t;
            let tt = t * t;
            let uu = u * u;
            let x = uu * p0.0 + 2.0 * u * t * p1.0 + tt * p2.0;
            let y = uu * p0.1 + 2.0 * u * t * p1.1 + tt * p2.1;
            (x as i16, y as i16)
        })
        .collect()
}

fn generate_quadratic_bezier_from_points(points: &[(f32, f32)], steps: usize) -> Vec<(i16, i16)> {
    if points.len() < 3 {
        return points.iter().map(|&(x, y)| (x as i16, y as i16)).collect();
    }

    let mut result = Vec::with_capacity((points.len() - 2) * steps + 1);

    for window in points.windows(3) {
        let bezier_points = quadratic_bezier_points(window[0], window[1], window[2], steps);
        result.extend(bezier_points.into_iter().skip(1));
    }

    if points.len() % 2 == 0 {
        let last_two = &points[points.len() - 2..];
        let control_point = (
            (last_two[0].0 + last_two[1].0) / 2.0,
            (last_two[0].1 + last_two[1].1) / 2.0,
        );
        let bezier_points = quadratic_bezier_points(last_two[0], control_point, last_two[1], steps);
        result.extend(bezier_points.into_iter().skip(1));
    }

    result
}


pub fn draw_spline_polygon(canvas: &mut Canvas<Window>, input_points: &[(f32, f32)], steps: usize, color: Color) {
    let smooth_points = generate_quadratic_bezier_from_points(input_points, steps);
    let (x_points, y_points): (Vec<_>, Vec<_>) = smooth_points.into_iter().unzip();
    canvas.filled_polygon(&x_points, &y_points, color).unwrap();
}
