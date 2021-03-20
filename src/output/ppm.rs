use crate::output::Output;
use crate::painter::Painter;

pub struct PPM {
    pub file: &'static str
}

impl Output for PPM {
    fn render(&self, painter: &dyn Painter) {
        use std::fs::File;
        use std::io::prelude::*;

        let width = painter.width();
        let height = painter.height();

        let mut file =
            File::create(self.file)
            .expect("Could not open file!");
        
        write!(file, "P3\n{} {}\n255\n", width, height).expect("Could not write to file!");

        for y in 0..height {
            for x in 0..width {
                let (r, g, b) = painter.color(x, y);
                write!(file, "{} {} {}\n", r, g, b)
                    .expect("Could not write to file!");
            }
        }
    }
}
