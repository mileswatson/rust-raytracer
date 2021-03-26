pub use self::parallel_painter::{ParallelPainter };
pub use self::sequential_painter::SequentialPainter;

mod sequential_painter;
mod parallel_painter;

use super::brush::Brush;
use super::canvas::Canvas;

pub trait Painter {
    fn paint(&self, brush: &dyn Brush, canvas: &mut dyn Canvas);
}
