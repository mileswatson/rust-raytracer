mod output;
mod painter;

use crate::output::Output;

fn main() {
    let _output = output::window::Window {};
    let _painter = painter::gradient::Gradient {
        width: 1280,
        height: 720
    };
    _output.render(&_painter);
}
