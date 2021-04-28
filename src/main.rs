extern crate ray;

use ray::{
    painting::{canvas::Window, painter::*},
    tracing::{materials::*, objects::*, Camera, Color, Point, Scene},
};

fn main() {
    let ground = &Sphere {
        center: Point::new(0., -100.5, 1.),
        radius: 100.,
        material: &Lambertian {
            albedo: Color::new(0.8, 0.8, 0.0),
        },
    };
    let center = &Sphere {
        center: Point::new(0., 0., 2.),
        radius: 0.5,
        material: &Lambertian {
            albedo: Color::new(0.7, 0.3, 0.3),
        },
    };
    let left = &Sphere {
        center: Point::new(-1., 0., 2.),
        radius: 0.5,
        material: &Metal {
            albedo: Color::new(0.8, 0.8, 0.8),
            fuzz: 0.1,
        },
    };
    let right = &Sphere {
        center: Point::new(1., 0., 2.),
        radius: 0.5,
        material: &Metal {
            albedo: Color::new(0.8, 0.6, 0.2),
            fuzz: 0.5,
        },
    };

    let tracer = Camera {
        scene: Scene {
            hittables: vec![ground, center, left, right],
        },
        samples: 100,
        max_depth: 25,
    };
    let mut window = Window {
        width: 900,
        height: 500,
        img: None,
    };
    let painter = ParallelPainter {};

    painter.paint(&tracer, &mut window);
    window.display();
}
