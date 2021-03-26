pub use self::gradient::Gradient;
pub use self::mandlebrot::Mandlebrot;

mod gradient;
mod mandlebrot;

pub trait Brush: Sync + Send {
    fn color(&self, width: u32, height: u32, x: u32, y: u32) -> (u8, u8, u8);
}
