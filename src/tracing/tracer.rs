use super::vec3::Vec3;
use crate::painting::brush::Brush;

pub struct Tracer {}

impl Tracer {
    fn direction(&self, width: f32, height: f32, x: f32, y: f32) -> Vec3 {
        let x = (2. * x - width) / width;
        let y = (height - 2. * y) / width;
        Vec3::new(x, y, 1.)
    }

    fn hit_sphere(center: Vec3, radius: f32, direction: Vec3) -> bool {
        let a = direction.dot(direction);
        let b = 2. * center.dot(direction);
        let c = center.dot(center) - radius * radius;
        let discriminant = b * b - 4. * a * c;
        discriminant > 0.
    }

    fn trace(&self, ray: Vec3) -> (f32, f32, f32) {
        if Tracer::hit_sphere(Vec3::new(0., 0., 2.), 0.5, ray) {
            (1., 0., 0.)
        } else {
            (0., 0., 0.)
        }
    }

    fn grade(&self, col: (f32, f32, f32)) -> (u8, u8, u8) {
        (
            (col.0 * 256.) as u8,
            (col.1 * 256.) as u8,
            (col.2 * 256.) as u8,
        )
    }
}

impl Brush for Tracer {
    fn color(&self, width: u32, height: u32, x: u32, y: u32) -> (u8, u8, u8) {
        let pos = self.direction(width as f32, height as f32, x as f32, y as f32);
        let col = self.trace(pos);
        self.grade(col)
    }
}
