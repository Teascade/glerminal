use glerminal::{TerminalBuilder, TextBuffer};

fn main() {
    let terminal = TerminalBuilder::new()
        .with_title("TextBuffer Usage Example")
        .with_dimensions((1280, 720))
        .build();
    let mut text_buffer;
    match TextBuffer::create(&terminal, (80, 24)) {
        Ok(buffer) => text_buffer = buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }

    text_buffer.write("Hello, ");
    text_buffer.cursor.style.fg_color = [0.0, 1.0, 0.0, 1.0];
    text_buffer.write("this is green, ");
    text_buffer.cursor.style.bg_color = [1.0, 0.0, 0.0, 1.0];
    text_buffer.write("and this is on red bg!");
    text_buffer.cursor.style = Default::default();
    text_buffer.cursor.move_to(0, 1);
    text_buffer.write("And now back to normal, on second row ");
    text_buffer.cursor.style.shakiness = 1.0;
    text_buffer.write("shaking!");

    terminal.flush(&mut text_buffer);

    while terminal.refresh() {
        terminal.draw(&text_buffer);
    }
}
