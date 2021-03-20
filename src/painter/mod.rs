
pub mod gradient;

pub trait Painter {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn color(&self, width: u32, height: u32, x: u32, y: u32) -> (u8, u8, u8);
}
