use super::Painter;
use crate::painting::{brush::Brush, canvas::Canvas};

use image::RgbaImage;

pub struct SequentialPainter {}

impl Painter for SequentialPainter {
    fn paint(&self, brush: &dyn Brush, canvas: &mut dyn Canvas) {
        let (width, height) = (canvas.width(), canvas.height());
        let mut buffer = RgbaImage::from_pixel(width, height, image::Rgba([0, 0, 0, 0]));

        use pbr::ProgressBar;
        let mut pb = ProgressBar::new((height * width) as u64);

        for y in 0..height {
            for x in 0..width {
                let (r, g, b) = brush.color(width, height, x, y);
                buffer.put_pixel(x, y, image::Rgba([r, g, b, 255]));
            }
            pb.add(width as u64);
        }
        pb.finish_print("done");

        canvas.draw(buffer);
    }
}
