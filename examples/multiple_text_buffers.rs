extern crate glerminal;

use glerminal::{TerminalBuilder, TextBuffer};

fn main() {
    let terminal = TerminalBuilder::new()
        .with_title("Multiple TextBuffer Example")
        .with_dimensions((1280, 720))
        .build();

    let mut text_buffer_bg;
    match TextBuffer::new(&terminal, (80, 24)) {
        Ok(buffer) => text_buffer_bg = buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }

    let mut text_buffer_overlay;
    match TextBuffer::new(&terminal, (40, 12)) {
        Ok(buffer) => text_buffer_overlay = buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }


    while terminal.refresh() {

        text_buffer_bg.clear();
        text_buffer_bg.move_cursor(0, 0);
        text_buffer_bg.write("On Background!");

        terminal.flush(&mut text_buffer_bg);

        text_buffer_overlay.clear();
        text_buffer_overlay.move_cursor(1, 0);
        text_buffer_overlay.change_cursor_bg_color([0.0, 0.0, 1.0, 0.5]);
        text_buffer_overlay.put_char('^');
        text_buffer_overlay.move_cursor(0, 1);
        text_buffer_overlay.change_cursor_bg_color([0.0; 4]);
        text_buffer_overlay.write("Above!");

        terminal.flush(&mut text_buffer_overlay);

        terminal.clear();
        terminal.draw_single(&text_buffer_bg);
        terminal.draw_single(&text_buffer_overlay);
    }
}
