use crate::tracing::{Color, HitRecord, Material, Ray, Vec3};

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, h: HitRecord) -> Option<(Color, Ray)> {
        let dot = ray.direction.dot(h.normal);
        if dot < 0. {
            let incident = ray.direction;
            let normal = h.normal;
            let direction = incident - 2. * incident.dot(normal) * normal;
            let fuzzed = direction + self.fuzz * Vec3::random_on_sphere();
            let scattered = Ray::new(h.point, fuzzed);
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
