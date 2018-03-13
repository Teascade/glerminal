use terminal::{Terminal, TerminalBuilder};
use text_buffer::TextBuffer;
use renderer;

mod terminal;
mod input;
mod text_buffer;
mod parser;
mod font;

use rand;

#[test]
fn test_for_gl_error() {
    let terminal = test_setup_open_terminal();
    let mut buffer = match TextBuffer::new(&terminal, (2, 2)) {
        Ok(buffer) => buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    };

    while terminal.refresh() {
        terminal.draw(&mut buffer);
        terminal.close();
    }

    assert_eq!(renderer::get_error(), None);
}

fn test_setup_open_terminal() -> Terminal {
    TerminalBuilder::new()
        .with_title("Simple window")
        .with_dimensions((1280, 720))
        .with_visibility(false)
        .build()
}

fn test_setup_text_buffer(dimensions: (i32, i32)) -> TextBuffer {
    let terminal = test_setup_open_terminal();
    match TextBuffer::new(&terminal, dimensions) {
        Ok(buffer) => buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }
}

fn run_multiple_times<F: Fn()>(f: F) {
    for _ in 0..10 {
        f();
    }
}

fn random_color() -> [f32; 4] {
    [
        rand::random::<f32>(),
        rand::random::<f32>(),
        rand::random::<f32>(),
        rand::random::<f32>(),
    ]
}
