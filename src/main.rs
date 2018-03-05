extern crate gl;
extern crate glutin;
extern crate libc;
extern crate png;
extern crate sfl_parser;

mod display;
mod renderer;
mod font;
mod text_buffer;
mod terminal;

use terminal::Terminal;

use std::time::{Duration, SystemTime};

fn main() {
    let mut terminal;
    match Terminal::new((1280, 720), (80, 25)) {
        Ok(term) => terminal = term,
        Err(error) => panic!(format!("Failed to initialize terminal: {}", error)),
    }

    let mut last_time = SystemTime::now();
    let mut frames = 0;

    terminal.write("Hello, World!");
    terminal.swap_buffers();

    while terminal.refresh() {
        terminal.draw();

        frames += 1;

        if last_time + Duration::new(1, 0) < SystemTime::now() {
            terminal.set_title(format!("Hello, World! FPS: {}", frames));
            frames = 0;
            last_time = SystemTime::now();
        }
    }
}
