use crate::brush::Brush;
use crate::canvas::Canvas;
use crate::painter::Painter;

use std::sync::Arc;
use image::{RgbaImage};

pub struct SequentialPainter {}

impl Painter for SequentialPainter {
    fn paint(&self, brush: Arc<dyn Brush>, canvas: Arc<dyn Canvas>) {
        let (width, height) = (canvas.width(), canvas.height());
        let mut buffer = RgbaImage::from_pixel(width, height, image::Rgba([0, 0, 0, 0]));

        use pbr::ProgressBar;
        let mut pb = ProgressBar::new(height as u64);
        pb.format("╢▌▌░╟");

        for y in 0..height {
            for x in 0..width {
                let (r, g, b) = brush.color(x, y);
                buffer.put_pixel(x, y, image::Rgba([r, g, b, 255]));
            }
            pb.inc();
        }

        pb.finish_print("done");

        canvas.display(buffer);

    }
}