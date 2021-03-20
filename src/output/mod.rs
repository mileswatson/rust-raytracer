pub mod window;

pub trait Output {
    fn render(&self, painter: &dyn crate::painter::Painter);
}
