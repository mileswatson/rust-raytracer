use super::Brush;

pub struct Gradient {}

impl Brush for Gradient {

    fn color(&self, width: u32, height: u32, x: u32, y: u32) -> (u8, u8, u8) {
        let (width, height, x, y) = (width as f32, height as f32, x as f32, y as f32);
        let (x, y) = (x/width, y/height);

        let r = x * 256.;
        let g = y * 256.;
        let b = x * y * 256.;
        (r as u8, g as u8, b as u8)
    }
}
