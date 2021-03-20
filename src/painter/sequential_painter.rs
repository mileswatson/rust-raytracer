use crate::brush::Brush;
use crate::canvas::Canvas;
use crate::painter::Painter;

use image::{RgbImage};

pub struct SequentialPainter {}

impl Painter for SequentialPainter {
    fn paint(&self, brush: &dyn Brush, canvas: &dyn Canvas) {
        let (width, height) = (canvas.width(), canvas.height());
        let mut buffer = RgbImage::from_pixel(width, height, image::Rgb([0, 0, 0]));

        use pbr::ProgressBar;
        let mut pb = ProgressBar::new(height as u64);
        pb.format("╢▌▌░╟");

        for y in 0..height {
            for x in 0..width {
                buffer.put_pixel(x, y, brush.color(x, y));
            }
            pb.inc();
        }

        pb.finish_print("done");

        canvas.display(buffer);

    }
}
