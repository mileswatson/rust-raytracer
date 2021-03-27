pub use self::mock::Mock;
pub use self::ppm::PPM;
pub use self::window::Window;

mod ppm;
mod window;

pub trait Canvas: Sync + Send {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn draw(&mut self, img: image::RgbaImage);
}

mod mock {
    pub struct Mock {
        pub width: u32,
        pub height: u32,
    }

    impl super::Canvas for Mock {
        fn width(&self) -> u32 {
            self.width
        }

        fn height(&self) -> u32 {
            self.height
        }

        fn draw(&mut self, _img: image::RgbaImage) {}
    }
}
