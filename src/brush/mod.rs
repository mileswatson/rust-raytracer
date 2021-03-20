pub mod gradient;

pub trait Brush {
    fn color(&self, x: u32, y: u32) -> image::Rgb<u8>;
}
