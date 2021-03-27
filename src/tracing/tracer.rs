use crate::painting::brush::Brush;
use crate::tracing::scene::*;
use crate::tracing::{Color, Ray, Vec3};

pub struct Tracer {
    pub sphere: Sphere,
}

impl Tracer {
    fn get_ray(&self, width: f32, height: f32, x: f32, y: f32) -> Ray {
        let x = (2. * x - width) / width;
        let y = (height - 2. * y) / width;
        Ray {
            origin: Vec3::new(0., 0., 0.),
            direction: Vec3::new(x, y, 1.),
        }
    }

    fn trace(&self, ray: Ray) -> Color {
        if let Some(h) = self.sphere.hit(ray, 1., 100.) {
            0.5 * (h.normal + Vec3::new(1., 1., 1.))
        } else {
            let t = 0.5 * (ray.direction.y + 1.);
            (1.0 - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
        }
    }

    fn grade(&self, col: Color) -> (u8, u8, u8) {
        (
            (col.x * 256.) as u8,
            (col.y * 256.) as u8,
            (col.z * 256.) as u8,
        )
    }
}

impl Brush for Tracer {
    fn color(&self, width: u32, height: u32, x: u32, y: u32) -> (u8, u8, u8) {
        let ray = self.get_ray(width as f32, height as f32, x as f32, y as f32);
        let col = self.trace(ray);
        self.grade(col)
    }
}
