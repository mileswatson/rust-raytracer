use crate::painter::Painter;

pub struct Gradient { 
    pub width: u32,
    pub height: u32
}

impl Painter for Gradient {
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn color(&self, x: u32, y: u32) -> (u8, u8, u8) {
        (x as u8, y as u8, 127)
    }
}
