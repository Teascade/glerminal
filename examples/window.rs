extern crate glerminal;

use glerminal::terminal::TerminalBuilder;
use glerminal::text_buffer::TextBuffer;

fn main() {
    let terminal = TerminalBuilder::new()
        .with_title("Simple window")
        .with_dimensions((1280, 720))
        .build();
    let text_buffer;
    match TextBuffer::new(&terminal, (80, 24)) {
        Ok(buffer) => text_buffer = buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }

    while terminal.refresh() {
        terminal.draw(&text_buffer);
    }
}
