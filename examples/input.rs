extern crate glerminal;

use glerminal::{Parser, TerminalBuilder, TextBuffer, VirtualKeyCode};

fn main() {
    let terminal = TerminalBuilder::new()
        .with_title("Input test")
        .with_dimensions((1280, 720))
        .build();
    let mut text_buffer;
    match TextBuffer::new(&terminal, (80, 24)) {
        Ok(buffer) => text_buffer = buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }

    let mut parser = Parser::new();
    parser.add_color("red", [1.0, 0.2, 0.2, 1.0]);
    parser.add_color("green", [0.2, 1.0, 0.2, 1.0]);
    update_text(&parser, &mut text_buffer, false);

    terminal.flush(&mut text_buffer);

    while terminal.refresh() {
        terminal.draw(&text_buffer);

        let events = terminal.get_current_events();
        if events.keyboard.was_just_pressed(VirtualKeyCode::Space) {
            update_text(&parser, &mut text_buffer, true);
            terminal.flush(&mut text_buffer);
        } else if events.keyboard.was_just_released(VirtualKeyCode::Space) {
            update_text(&parser, &mut text_buffer, false);
            terminal.flush(&mut text_buffer);
        }
    }
}

fn update_text(parser: &Parser, text_buffer: &mut TextBuffer, pressed: bool) {
    text_buffer.clear();
    text_buffer.move_cursor(0, 0);
    let mut color = "red";
    if pressed {
        color = "green";
    }
    parser.write(
        text_buffer,
        format!("Spacebar is pressed: [fg={}]{}[/fg]", color, pressed),
    );
}
