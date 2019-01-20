fn main() {
    use glerminal::menu_systems::{InterfaceItem, TextItem};
    use glerminal::{TerminalBuilder, TextBuffer};

    // Initialize terminal and text buffer
    let terminal = TerminalBuilder::new().build();
    let mut text_buffer;
    match TextBuffer::create(&terminal, (80, 24)) {
        Ok(buffer) => text_buffer = buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }

    // Create a button
    let mut button = TextItem::new("Press me")
        .with_is_button(true)
        .with_focused(true);

    let processor = glerminal::text_processing::DefaultProcessor;

    while terminal.refresh() {
        button.handle_events(&terminal.get_current_events());
        if button.was_just_pressed() {
            button.set_text("Pressed!");
        }
        button.update(terminal.delta_time(), &processor);
        if button.get_base().dirty {
            button.draw(&mut text_buffer);
            terminal.flush(&mut text_buffer);
        }
        terminal.draw(&text_buffer);
    }
}
