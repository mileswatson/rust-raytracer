mod brush;
mod canvas;
mod painter;

use painter::Painter;
use std::sync::Arc;

fn main() {
    let window = canvas::window::Window { width: 1920, height: 1080 };
    let gradient = brush::gradient::Gradient {};
    let _painter = painter::sequential_painter::SequentialPainter {};

    _painter.paint(Arc::new(gradient), Arc::new(window));
}
