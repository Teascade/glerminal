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

use std::time::{Duration, SystemTime};

fn main() {
    let mut display: Display = Display::new("Hello, World!", (720, 720));

    let font = Font::load(
        "fonts/vcr_osd_mono_regular_48.png",
        "fonts/vcr_osd_mono_regular_48.sfl",
    );

    let mut last_time = SystemTime::now();
    let mut frames = 0;

    let program = renderer::create_program();
    let renderable_box = renderer::Renderable::new_box(program, font, 'ä');

    while display.refresh() {
        renderer::clear();
        renderer::draw(program, display.proj_matrix, &renderable_box);

        frames += 1;

        if last_time + Duration::new(1, 0) < SystemTime::now() {
            display.set_title(&format!("Hello, World! FPS: {}", frames));
            frames = 0;
            last_time = SystemTime::now();
        }
    }
}
