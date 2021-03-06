use glerminal::{TerminalBuilder, TextBuffer};

fn main() {
    let terminal = TerminalBuilder::new()
        .with_title("Multiple TextBuffer Example")
        .with_dimensions((1280, 720))
        .build();

    let mut text_buffer_bg;
    match TextBuffer::create(&terminal, (80, 24)) {
        Ok(buffer) => text_buffer_bg = buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }

    let mut text_buffer_overlay;
    match TextBuffer::create(&terminal, (40, 12)) {
        Ok(buffer) => text_buffer_overlay = buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }

    while terminal.refresh() {
        text_buffer_bg.clear();
        text_buffer_bg.cursor.move_to(0, 0);
        text_buffer_bg.write("On Background!");

        terminal.flush(&mut text_buffer_bg);

        text_buffer_overlay.clear();
        text_buffer_overlay.cursor.move_to(1, 0);
        text_buffer_overlay.cursor.style.bg_color = [0.0, 0.0, 1.0, 0.5];
        text_buffer_overlay.put_char('^');
        text_buffer_overlay.cursor.move_to(0, 1);
        text_buffer_overlay.cursor.style = Default::default();
        text_buffer_overlay.write("Above!");

        terminal.flush(&mut text_buffer_overlay);

        terminal.draw(&text_buffer_bg);
        terminal.draw(&text_buffer_overlay);
    }
}
