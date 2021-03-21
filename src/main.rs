extern crate raytracing;

use raytracing::{
    brush::Gradient,
    canvas::Window,
    painter::*
};

fn main() {
    let gradient = Gradient {};
    let mut window = Window { width: 1920, height: 1080, img: None };
    let seq = SequentialPainter {};
    let par = ParallelPainter {};

    seq.paint(&gradient, &mut window);
    par.paint(&gradient, &mut window);
}
