pub mod window;
pub mod ppm;

pub trait Output {
    fn render(&self, painter: &dyn crate::painter::Painter);
}
