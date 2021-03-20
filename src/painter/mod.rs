pub mod gradient;

pub trait Painter {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn color(&self, x: u32, y: u32) -> (u8, u8, u8);
}
