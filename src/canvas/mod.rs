pub mod window;
pub mod ppm;

use image::RgbaImage;

pub trait Canvas: Sync + Send {
    fn display(&self, img: RgbaImage);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}
