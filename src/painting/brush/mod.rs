pub use self::gradient::Gradient;
pub use self::mandlebrot::Mandlebrot;

mod gradient;
mod mandlebrot;

/// Represents a brush that maps pixels to colors.
pub trait Brush: Sync {
    /// Calculates the color of a specific pixel.
    /// Should be defined for x: [0, width-1] and y: [0, height-1].
    fn color(&self, width: u32, height: u32, x: u32, y: u32) -> (u8, u8, u8);
}
