extern crate ray;

use ray::tracing::scene::Sphere;
use ray::{
    painting::{canvas::Window, painter::*},
    tracing::{Color, Point, Tracer},
};

fn main() {
    let tracer = Tracer {
        sphere: Sphere {
            center: Point::new(0., 0., 2.),
            color: Color::new(1., 0., 0.),
            radius: 0.5,
        },
    };
    let mut window = Window {
        width: 1800,
        height: 1000,
        img: None,
    };
    let painter = ParallelPainter {};

    painter.paint(&tracer, &mut window);
    window.display();
}
