use glerminal::{MouseButton, Parser, TerminalBuilder, TextBuffer, VirtualKeyCode};

fn main() {
    let terminal = TerminalBuilder::new()
        .with_title("Input test")
        .with_dimensions((1280, 720))
        .build();
    let mut text_buffer;
    match TextBuffer::create(&terminal, (80, 24)) {
        Ok(buffer) => text_buffer = buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }

    let mut parser = Parser::new();
    parser.add_color("red", [1.0, 0.2, 0.2, 1.0]);
    parser.add_color("green", [0.2, 1.0, 0.2, 1.0]);

    update_texts(&parser, &mut text_buffer, false, false);

    text_buffer.cursor.move_to(0, 3);
    text_buffer.write("cursor pos: None");

    terminal.flush(&mut text_buffer);

    let mut last_position = None;

    while terminal.refresh() {
        terminal.draw(&text_buffer);

        let events = terminal.get_current_events();

        let loc = events.cursor.get_location(&text_buffer);

        if events.keyboard.was_just_pressed(VirtualKeyCode::Space)
            || events.keyboard.was_just_released(VirtualKeyCode::Space)
            || events.mouse.was_just_pressed(MouseButton::Left)
            || events.mouse.was_just_released(MouseButton::Left)
            || (events.cursor.cursor_just_moved() && loc != last_position)
        {
            text_buffer.clear();

            // Show button inputs
            let space = events.keyboard.is_pressed(VirtualKeyCode::Space);
            let lmb = events.mouse.is_pressed(MouseButton::Left);
            update_texts(&parser, &mut text_buffer, space, lmb);

            // Show cursor position
            let text = format!("cursor pos: {:?}", loc);
            text_buffer.cursor.move_to(0, 3);
            text_buffer.write(text);

            // Draw a blue bg where the cursor is
            if let Some(loc) = loc {
                if let Some(c) = text_buffer.get_character(loc.0, loc.1) {
                    text_buffer.cursor.move_to(loc.0, loc.1);
                    text_buffer.cursor.style.bg_color = [0.2, 0.2, 1.0, 1.0];
                    text_buffer.put_char(c.get_char());
                    text_buffer.cursor.style = Default::default();
                }
            }
            last_position = loc;

            terminal.flush(&mut text_buffer);
        }
    }
}

fn update_texts(parser: &Parser, text_buffer: &mut TextBuffer, spacebar: bool, lmb: bool) {
    text_buffer.clear();
    let colors = ["red", "green"];

    text_buffer.cursor.move_to(0, 0);
    parser.write(
        text_buffer,
        format!(
            "Spacebar is pressed: [fg={}]{}[/fg]",
            colors[if spacebar == true { 1 } else { 0 }],
            spacebar
        ),
    );
    text_buffer.cursor.move_to(0, 1);
    parser.write(
        text_buffer,
        format!(
            "Left mouse button is pressed: [fg={}]{}[/fg]",
            colors[if lmb == true { 1 } else { 0 }],
            lmb
        ),
    );
}
