use crate::tracing::{Point, Ray, Vec3};

pub use self::sphere::Sphere;

pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub distance: f32,
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

mod sphere;
