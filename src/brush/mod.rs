pub mod gradient;

pub trait Brush: Sync + Send {
    fn color(&self, x: u32, y: u32) -> (u8, u8, u8);
}
