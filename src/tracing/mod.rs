pub use self::ray::Ray;
pub use self::tracer::Tracer;
pub use self::vec3::{Color, Point, Vec3};

mod ray;
pub mod scene;
mod tracer;
mod vec3;
