use crate::brush::Brush;
use crate::canvas::Canvas;

pub mod sequential_painter;

pub trait Painter {
    fn paint(&self, brush: &dyn Brush, canvas: &dyn Canvas);
}
