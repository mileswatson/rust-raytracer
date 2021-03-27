extern crate ray;

use ray::{
    painting::{canvas::Window, painter::*},
    tracing::Tracer,
};

fn main() {
    let tracer = Tracer {};
    let mut window = Window {
        width: 1800,
        height: 1000,
        img: None,
    };
    let painter = ParallelPainter {};

    painter.paint(&tracer, &mut window);
    window.display();
}
