extern crate gl;
extern crate glutin;
extern crate libc;
extern crate png;
extern crate sfl_parser;

mod display;
mod renderer;
mod font;

use display::Display;
use font::Font;

// TODO: Use png-module to load the png font image.
// TODO: Try to use .fnt and make renderer use that font

fn main() {
    let mut display: Display = Display::new("Hello, World!");

    let font = Font::load(
        "fonts/vcr_osd_mono_regular_48.png",
        "fonts/vcr_osd_mono_regular_48.sfl",
    );

    let program = renderer::create_program();
    let renderable_box = renderer::Renderable::new_box(program, font, 'ä');
    while display.refresh() {
        renderer::clear();
        renderer::draw(program, &renderable_box);
    }
}
