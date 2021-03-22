use super::Canvas;
use image::{RgbaImage};

use std::fs::File;
use std::io::prelude::*;

pub struct PPM {
    pub width: u32,
    pub height: u32,
    pub file: &'static str,
    pub img: Option<RgbaImage>,
}

impl PPM {
    pub fn save(&mut self) {
        let img = self.img.as_ref().expect("Image has not been rendered!");

        let (width, height) = img.dimensions();

        use pbr::ProgressBar;
        let mut pb = ProgressBar::new(height as u64);
        pb.format("╢▌▌░╟");

        let mut file =
            File::create(self.file)
            .expect("Could not open file!");
        
        write!(file, "P3\n{} {}\n255\n", width, height).expect("Could not write to file!");

        for y in 0..height {
            for x in 0..width {
                let color = img.get_pixel(x, y);
                writeln!(file, "{} {} {}", color[0], color[1], color[2])
                    .expect("Could not write to file!");
            }
            pb.inc();
        }

        pb.finish_print("done");
    }
}

impl Canvas for PPM {
    fn width(&self) -> u32 { self.width }

    fn height(&self) -> u32 {self.height }

    fn draw(&mut self, img: RgbaImage) { self.img = Some(img); }
}
