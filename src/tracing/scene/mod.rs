use crate::tracing::{Point, Ray, Vec3};

pub use self::sphere::Sphere;

pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub distance: f32,
    pub front_face: bool,
}

impl HitRecord {
    fn new(ray: Ray, distance: f32, normal: Vec3) -> HitRecord {
        let front_face = ray.direction.dot(normal) < 0.;
        HitRecord {
            point: ray.at(distance),
            normal: if front_face { normal } else { -normal },
            distance,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

mod sphere;
