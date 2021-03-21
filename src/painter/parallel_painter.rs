use crate::brush::Brush;
use crate::canvas::Canvas;
use super::Painter;

use std::sync::Mutex;
use std::vec::Vec;
use image::RgbaImage;
use rayon::prelude::*;

pub struct ParallelPainter {}

fn generate_vec(brush: &dyn Brush, y: u32, width: u32) -> Vec<u8> {

    let mut pixels = std::vec::Vec::with_capacity(width as usize * 4);

    for x in 0..width {
        let (r, g, b) = brush.color(x, y);
        pixels.push(r);
        pixels.push(g);
        pixels.push(b);
        pixels.push(255);
    }

    pixels
}

impl Painter for ParallelPainter {
    fn paint(&self, brush: &dyn Brush, canvas: &mut dyn Canvas) {
        let (width, height) = (canvas.width(), canvas.height());

        use pbr::ProgressBar;
        let pb = Mutex::new(ProgressBar::new((height*width) as u64));

        let pixels: Vec<Vec<u8>> = (0..height).into_par_iter()
            .map(|y| {
                let v = generate_vec(brush, y, width);
                pb.lock().unwrap().add(width as u64);
                v
            }).collect();
        
        pb.lock().unwrap().finish_print("done");

        let pixels = pixels.concat();
        let img = RgbaImage::from_vec(canvas.width(), canvas.height(), pixels).expect("Failed to create image!");

        canvas.draw(img);
    }
}
