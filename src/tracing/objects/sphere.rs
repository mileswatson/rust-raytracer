use crate::tracing::{HitRecord, Hittable, Ray, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let co = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = co.dot(ray.direction);
        let c = co.length_squared() - self.radius * self.radius;

        #[allow(clippy::suspicious_operation_groupings)]
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }

        let root = (-half_b - discriminant.sqrt()) / a;
        if root < t_min || t_max < root {
            return None;
        }

        let point = ray.at(root);
        let normal = (point - self.center).unit();
        Some(HitRecord::new(ray, root, normal))
    }
}
