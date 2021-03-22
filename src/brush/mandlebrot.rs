use super::Brush;

pub struct Mandlebrot {
    pub center: (f32, f32),
    pub scale: f32,
    pub iters: u32,
    pub max: f32,
}

impl Brush for Mandlebrot {

    fn color(&self, width: u32, height: u32, x: u32, y: u32) -> (u8, u8, u8) {
        let (width, height, x, y) = (width as f32, height as f32, x as f32, y as f32);

        let (x, y) = (x/width, y/width);
        let c = Complex {
            r: (x-0.5)*self.scale+self.center.0,
            i: (height/(2.*width)-y)*-self.scale-self.center.1
        };

        let mut z = Complex { r: 0., i: 0. };
        for _ in 0..self.iters {
            z = z*z + c;
            if z.r > self.max || z.i > self.max {
                return (0, 0, 0);
            }
        }
        (255, 255, 255)
    }
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    r: f32,
    i: f32
}

impl std::ops::Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            r: self.r + other.r,
            i: self.i + other.i
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            // Square self.r
            r: self.r * other.r - self.i * other.i,
            i: self.r * other.i + self.i * other.r
        }
    }
}
