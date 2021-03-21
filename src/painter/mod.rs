pub use self::parallel_painter::{ParallelPainter };
pub use self::sequential_painter::SequentialPainter;

mod sequential_painter;
mod parallel_painter;

use crate::brush::Brush;
use crate::canvas::Canvas;

pub trait Painter {
    fn paint(&self, brush: &dyn Brush, canvas: &mut dyn Canvas);
}
