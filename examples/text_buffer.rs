extern crate adragaria_remastered;

use adragaria_remastered::terminal::TerminalBuilder;
use adragaria_remastered::text_buffer::TextBuffer;

fn main() {
    let terminal = TerminalBuilder::new()
        .with_title("TextBuffer Usage Example")
        .with_dimensions((1280, 720))
        .build();
    let mut text_buffer;
    match TextBuffer::new(&terminal, (80, 24)) {
        Ok(buffer) => text_buffer = buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }

    text_buffer.write("Hello, ");
    text_buffer.change_cursor_fg_color([0.0, 1.0, 0.0, 1.0]);
    text_buffer.write("this is green, ");
    text_buffer.change_cursor_bg_color([1.0, 0.0, 0.0, 1.0]);
    text_buffer.write("and this is on red bg!");
    text_buffer.change_cursor_fg_color([1.0, 1.0, 1.0, 1.0]);
    text_buffer.change_cursor_bg_color([0.0, 0.0, 0.0, 0.0]);
    text_buffer.move_cursor(0, 1);
    text_buffer.write("And now back to normal, on second row ");
    text_buffer.change_cursor_shakiness(1.0);
    text_buffer.write("shaking!");

    terminal.flush(&mut text_buffer);

    while terminal.refresh() {
        terminal.draw(&text_buffer);
    }
}
