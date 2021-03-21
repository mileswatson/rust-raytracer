use super::Canvas;
use image::{RgbaImage};

pub struct Window {
    pub width: u32,
    pub height: u32,
    pub img: Option<RgbaImage>,
}

impl Window {
    pub fn display(&mut self, img: RgbaImage) {

        let (width, height) = img.dimensions();

        let mut window : piston_window::PistonWindow =
            piston_window::WindowSettings::new("Raytracer", [width, height])
            .exit_on_esc(true)
            .build()
            .unwrap();
        
        let tex = piston_window::Texture::from_image(
            &mut window.create_texture_context(),
            &img,
            &piston_window::TextureSettings::new())
            .unwrap();

        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, _| {
                piston_window::clear([1.0; 4], g);
                piston_window::image(&tex, c.transform, g);
            });
        }
    }
}

impl Canvas for Window {

    fn width(&self) -> u32 { self.width }

    fn height(&self) -> u32 {self.height }

    fn draw(&mut self, img: RgbaImage) { self.img = Some(img); }
}
