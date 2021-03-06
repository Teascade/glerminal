use crate::renderer;
use crate::terminal::{Terminal, TerminalBuilder};
use crate::text_buffer::{Color, TextBuffer};

mod events;
mod font;
mod parser;
mod terminal;
mod text_buffer;

#[cfg(feature = "menu_systems")]
mod menu_systems;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

#[test]
fn gl_error() {
    let terminal = test_setup_open_terminal();
    let buffer = match TextBuffer::create(&terminal, (2, 2)) {
        Ok(buffer) => buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    };

    while terminal.refresh() {
        terminal.draw(&buffer);
        terminal.close();
    }

    assert_eq!(renderer::get_error(terminal.headless), None);
}

fn test_setup_open_terminal() -> Terminal {
    TerminalBuilder::new()
        .with_title("Simple window")
        .with_dimensions((1280, 720))
        .with_headless(true)
        .build()
}

fn test_setup_text_buffer(dimensions: (u32, u32)) -> TextBuffer {
    let terminal = test_setup_open_terminal();
    match TextBuffer::create(&terminal, dimensions) {
        Ok(buffer) => buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }
}

fn test_setup_text_buffer_with_terminal(dimensions: (u32, u32)) -> (TextBuffer, Terminal) {
    let terminal = test_setup_open_terminal();
    (
        match TextBuffer::create(&terminal, dimensions) {
            Ok(buffer) => buffer,
            Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
        },
        terminal,
    )
}

fn run_multiple_times<F: Fn()>(how_many_times: u32, f: F) {
    for _ in 0..how_many_times {
        f();
    }
}

fn random_color() -> Color {
    [
        rand::random::<f32>(),
        rand::random::<f32>(),
        rand::random::<f32>(),
        rand::random::<f32>(),
    ]
}

fn random_char() -> char {
    thread_rng().sample_iter(&Alphanumeric).next().unwrap()
}

fn random_text(len: u32) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len as usize)
        .collect()
}
