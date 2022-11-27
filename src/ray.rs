use crate::{color::Color, point::Point, vec3::Vector3};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + (t * self.direction)
    }

    pub fn ray_color(&self) -> Color {
        let ray_direction = self.direction.unit();
        let t = 0.5 * (ray_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
