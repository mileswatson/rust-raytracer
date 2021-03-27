use super::Brush;

// A simple brush that calculates a gradient.
pub struct Gradient {}

impl Brush for Gradient {
    fn color(&self, width: u32, height: u32, x: u32, y: u32) -> (u8, u8, u8) {
        let (width, height, x, y) = (width as f32, height as f32, x as f32, y as f32);
        let (x, y) = (x / width, y / height);

        let red = x * 256.;
        let green = y * 256.;
        let blue = x * y * 256.;
        (red as u8, green as u8, blue as u8)
    }
}
