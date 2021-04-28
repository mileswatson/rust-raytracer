extern crate ray;

use ray::{
    painting::{canvas::Window, painter::*},
    tracing::{materials::*, objects::*, Camera, Color, Point, Scene},
};

fn main() {
    let s = &Sphere {
        center: Point::new(0., 0., 2.),
        radius: 0.5,
        material: &Lambertian {
            albedo: Color {
                x: 0.3,
                y: 0.5,
                z: 0.1,
            },
        },
    };
    let world = &Sphere {
        center: Point::new(0., -100.5, 1.),
        radius: 100.,
        material: &Lambertian {
            albedo: Color {
                x: 0.3,
                y: 0.1,
                z: 0.2,
            },
        },
    };
    let tracer = Camera {
        scene: Scene {
            hittables: vec![s, world],
        },
        samples: 10,
        max_depth: 25,
    };
    let mut window = Window {
        width: 450,
        height: 250,
        img: None,
    };
    let painter = ParallelPainter {};

    painter.paint(&tracer, &mut window);
    window.display();
}
