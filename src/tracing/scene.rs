use crate::tracing::{Color, Point, Ray, Vec3};

pub struct HitRecord<'a> {
    pub point: Point,
    pub normal: Vec3,
    pub distance: f32,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl HitRecord<'_> {
    pub fn new(ray: Ray, distance: f32, normal: Vec3, material: &dyn Material) -> HitRecord {
        let front_face = ray.direction.dot(normal) < 0.;
        HitRecord {
            point: ray.at(distance),
            normal: if front_face { normal } else { -normal },
            distance,
            front_face,
            material,
        }
    }
}

pub trait Material: Sync {
    fn scatter(&self, ray: Ray, hit: HitRecord) -> Option<(Color, Ray)>;
}

pub trait Hittable: Sync {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Scene<'a> {
    pub hittables: Vec<&'a dyn Hittable>,
}

impl Hittable for Scene<'_> {
    fn hit(&self, ray: Ray, t_min: f32, mut t_max: f32) -> Option<HitRecord> {
        let mut x = None;
        for h in &self.hittables {
            let hit = h.hit(ray, t_min, t_max);
            x = match hit {
                None => x,
                Some(record) => {
                    t_max = record.distance;
                    Some(record)
                }
            }
        }
        x
    }
}
