extern crate raytracing;

use raytracing::painting::{
    brush::Mandlebrot,
    canvas::Window,
    painter::*
};

fn main() {
    let gradient = Mandlebrot {
        center: (-1., 0.),
        scale: 4.,
        iters: 25,
        max: 1000.
    };
    let mut window = Window { width: 1800, height: 1000, img: None };
    let painter = ParallelPainter {};

    painter.paint(&gradient, &mut window);
    window.display();
}
