pub mod window;
pub mod ppm;

use image::RgbImage;

pub trait Canvas {
    fn display(&self, img: RgbImage);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}
