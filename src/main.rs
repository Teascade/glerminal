extern crate gl;
extern crate glutin;
extern crate libc;
extern crate png;
extern crate sfl_parser;

mod display;
mod renderer;
mod font;
mod text_buffer;
mod input;
mod terminal;

use glutin::VirtualKeyCode;

use terminal::TerminalBuilder;
use text_buffer::TextBuffer;
use font::Font;

use std::time::{Duration, SystemTime};

fn main() {
    let mut terminal = TerminalBuilder::new()
        .with_title("Testi!")
        .with_dimensions((1280, 720))
        .build();
    let mut text_buffer;
    match TextBuffer::new(&terminal, (80, 24)) {
        Ok(buffer) => text_buffer = buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }

    let mut last_time = SystemTime::now();
    let mut frames = 0;

    text_buffer.write("Hello, World! Test-thing");
    text_buffer.move_cursor(0, 1);
    text_buffer.write("------------------------");
    text_buffer.move_cursor(31, 12);
    text_buffer.write("I AM IN THE CENTER");

    terminal.set_debug(false);

    let mut debug = false;

    while terminal.refresh() {
        terminal.flush(&mut text_buffer);
        terminal.draw(&text_buffer);
        frames += 1;

        let input = terminal.get_current_input();
        if input.was_just_pressed(VirtualKeyCode::F3) {
            debug = !debug;
            terminal.set_debug(debug);
        }

        if last_time + Duration::new(1, 0) < SystemTime::now() {
            terminal.set_title(format!("Hello, World! FPS: {}", frames));

            frames = 0;
            last_time = SystemTime::now();
        }
    }
}
