use crate::brush::Brush;
use crate::canvas::Canvas;

use std::sync::Arc;

pub mod sequential_painter;
pub mod parallel_painter;

pub trait Painter {
    fn paint(&self, brush: Arc<dyn Brush>, canvas: Arc<dyn Canvas>);
}
