pub use self::camera::Camera;
pub use self::ray::Ray;
pub use self::vec3::{Color, Point, Vec3};
pub use scene::{HitRecord, Hittable, Material, Scene};

mod camera;
pub mod materials;
pub mod objects;
mod ray;
mod scene;
mod vec3;
