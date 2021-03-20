use crate::output::Output;
use crate::painter::Painter;

pub struct Window {}

impl Output for Window {
    fn render(&self, painter: &dyn Painter) {
        use image::RgbaImage;

        let width = painter.width();
        let height = painter.height();

        let mut frame_buffer =
            RgbaImage::from_pixel(
                width, height, image::Rgba([0, 0, 255, 255]));

        let mut window : piston_window::PistonWindow =
            piston_window::WindowSettings::new("Raytracer", [width, height])
            .exit_on_esc(true)
            .build()
            .unwrap();
        
        for (x, y, pixel) in frame_buffer.enumerate_pixels_mut() {
            let color = painter.color(width, height, x, y);
            *pixel = image::Rgba([color.0, color.1, color.2, 255]);
        }
        let tex = piston_window::Texture::from_image(
            &mut window.create_texture_context(),
            &frame_buffer,
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
