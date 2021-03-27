use crate::tracing::vec3::*;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Ray {
        Self {
            origin,
            direction: direction.unit(),
        }
    }

    pub fn at(self, t: f32) -> Point {
        self.origin + t * self.direction
    }
}
