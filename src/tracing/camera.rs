use crate::painting::brush::Brush;
use crate::tracing::scene::*;
use crate::tracing::{Color, Ray, Vec3};

extern crate rand;
use rand::Rng;

pub struct Camera<'a> {
    pub scene: Scene<'a>,
    pub samples: u32,
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

fn clamp_color(col: Color) -> (u8, u8, u8) {
    (
        (clamp(col.x, 0., 0.999) * 256.) as u8,
        (clamp(col.y, 0., 0.999) * 256.) as u8,
        (clamp(col.z, 0., 0.999) * 256.) as u8,
    )
}

impl Camera<'_> {
    fn get_ray(&self, width: f32, height: f32, x: f32, y: f32) -> Ray {
        let x = (2. * x - width) / width;
        let y = (height - 2. * y) / width;
        Ray {
            origin: Vec3::new(0., 0., 0.),
            direction: Vec3::new(x, y, 1.),
        }
    }

    fn trace(&self, ray: Ray) -> Color {
        if let Some(h) = self.scene.hit(ray, 1., 100.) {
            0.5 * (h.normal + Vec3::new(1., 1., 1.))
        } else {
            let t = 0.5 * (ray.direction.y + 1.);
            (1.0 - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
        }
    }

    fn get_color(&self, width: f32, height: f32, x: f32, y: f32) -> (u8, u8, u8) {
        let mut colors = vec![];
        let mut rng = rand::thread_rng();
        for _ in 0..self.samples {
            let x = x + rng.gen::<f32>();
            let y = y + rng.gen::<f32>();
            let ray = self.get_ray(width, height, x, y);
            let col = self.trace(ray);
            colors.push(col);
        }
        let sum = colors
            .iter()
            .fold(Color::new(0., 0., 0.), |acc, val| acc + *val);
        let avg = sum / colors.len() as f32;
        clamp_color(avg)
    }
}

impl Brush for Camera<'_> {
    fn color(&self, width: u32, height: u32, x: u32, y: u32) -> (u8, u8, u8) {
        self.get_color(width as f32, height as f32, x as f32, y as f32)
    }
}
