use crate::tracing::{Color, HitRecord, Material, Ray, Vec3};

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, h: HitRecord) -> Option<(Color, Ray)> {
        let mut direction = h.normal + Vec3::random_on_sphere();
        if direction.near_zero() {
            direction = h.normal;
        }
        let scattered = Ray::new(h.point, direction);
        Some((self.albedo, scattered))
    }
}
