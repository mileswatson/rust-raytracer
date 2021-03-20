mod output;
mod painter;

use crate::output::Output;

fn main() {
    let _output = output::window::Window {};
    let _painter = painter::gradient::Gradient {
        width: 256,
        height: 128
    };
    _output.render(&_painter);
}
