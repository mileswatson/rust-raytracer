use crate::brush::Brush;

pub struct Gradient {}

impl Brush for Gradient {

    fn color(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let r = x as u8;
        let g = y as u8;
        let b = (x+y) as u8;
        (r, g, b)
    }
}
