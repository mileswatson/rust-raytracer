use crate::tracing::{HitRecord, Hittable, Material, Ray, Vec3};

pub struct Sphere<'a> {
    pub center: Vec3,
    pub radius: f32,
    pub material: &'a dyn Material,
}

impl Hittable for Sphere<'_> {
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
        let normal = if self.radius < 0. { -normal } else { normal };
        Some(HitRecord::new(ray, root, normal, self.material))
    }
}
