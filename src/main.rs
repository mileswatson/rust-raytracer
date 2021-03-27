extern crate ray;

use ray::tracing::scene::Scene;
use ray::tracing::scene::Sphere;
use ray::{
    painting::{canvas::Window, painter::*},
    tracing::{Point, Tracer},
};

fn main() {
    let s = &Sphere {
        center: Point::new(0., 0., 2.),
        radius: 0.5,
    };
    let world = &Sphere {
        center: Point::new(0., -100.5, 1.),
        radius: 100.,
    };
    let tracer = Tracer {
        scene: Scene {
            hittables: vec![s, world],
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
