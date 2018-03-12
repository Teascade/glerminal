extern crate gl;
extern crate glutin;
extern crate libc;
extern crate png;
extern crate regex;
extern crate sfl_parser;

mod display;
mod renderer;
mod font;
mod text_buffer;
mod parser;
mod input;
mod terminal;

use terminal::TerminalBuilder;
use text_buffer::TextBuffer;
use parser::Parser;

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

    let mut parser = Parser::new();
    parser.add_color("red", [1.0, 0.2, 0.2, 1.0]);
    parser.add_color("green", [0.2, 1.0, 0.2, 1.0]);
    parser.write(
        &mut text_buffer,
        "Hello, [fg=red]Mastodon[/fg]! And here is a [shake=0.7][fg=green]shakey boy[/fg][/shake].",
    );

    terminal.flush(&mut text_buffer);

    terminal.set_debug(false);

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
