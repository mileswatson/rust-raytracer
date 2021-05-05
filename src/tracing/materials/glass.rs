use crate::tracing::{Color, HitRecord, Material, Ray};

pub struct Glass {
    pub albedo: Color,
    pub refractive_index: f32,
}

fn reflectance(cos: f32, ratio: f32) -> f32 {
    let r0 = (1. - ratio) / (1. + ratio);
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cos).powi(5)
}

impl Material for Glass {
    fn scatter(&self, ray: Ray, h: HitRecord) -> Option<(Color, Ray)> {
        let ratio = if h.front_face {
            self.refractive_index
        } else {
            1. / self.refractive_index
        };

        let a = -h.normal.dot(ray.direction);
        let o = (1. - a * a).sqrt();
        let direction = if (o / ratio > 1.) || (reflectance(a, 1. / ratio) > rand::random::<f32>())
        {
            ray.direction + 2. * a * h.normal
        } else {
            let k = (a * a + ratio * ratio - 1.).sqrt() - a;
            ray.direction - k * h.normal
        };
        let scattered = Ray::new(h.point, direction);
        Some((self.albedo, scattered))
    }
}
