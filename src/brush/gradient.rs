use crate::brush::Brush;

pub struct Gradient {}

impl Brush for Gradient {

    fn color(&self, x: u32, y: u32) -> image::Rgb<u8> {
        let r = x as u8;
        let g = y as u8;
        let b = (x+y) as u8;
        let mut x = 0;
        for _ in 0..100 {
            x += 1;
        }
        image::Rgb([r, g, b])
    }
}
