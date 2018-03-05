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

use terminal::TerminalBuilder;
use text_buffer::TextBuffer;

use std::time::{Duration, SystemTime};

fn main() {
    let mut terminal = TerminalBuilder::new()
        .with_title("Testi!")
        .with_dimensions((1280, 720))
        .build();
    let mut text_buffer;
    match TextBuffer::new(&terminal, (80, 25)) {
        Ok(buffer) => text_buffer = buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }

    let mut last_time = SystemTime::now();
    let mut frames = 0;

    text_buffer.write("Hello, World!");
    terminal.flush(&text_buffer);

    while terminal.refresh() {
        terminal.draw(&text_buffer);
        frames += 1;

        if last_time + Duration::new(1, 0) < SystemTime::now() {
            terminal.set_title(format!("Hello, World! FPS: {}", frames));
            frames = 0;
            last_time = SystemTime::now();
        }
    }
}
