use glerminal::menu_systems::{Dialog, Filter, Menu, MenuList, MenuPosition, TextInput, TextItem};
use glerminal::{Parser, TerminalBuilder, TextBuffer};

fn main() {
    let terminal = TerminalBuilder::new()
        .with_title("Simple window")
        .with_dimensions((1280, 720))
        .build();
    let mut text_buffer;
    match TextBuffer::create(&terminal, (80, 24)) {
        Ok(buffer) => text_buffer = buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }

    let filter = Filter::empty_filter()
        .with_basic_latin_characters()
        .with_basic_numerals()
        .with_latin_1_supplement()
        .with_latin_extended_a()
        .with_basic_special_symbols();

    let mut title = TextItem::new("Menu systems + parser example").with_max_width(40);

    let mut button = TextItem::new("Times pressed: [fg=red]0[/fg]").with_is_button(true);
    let mut times_pressed = 0;

    let mut test_input = TextInput::new(None, None)
        .with_prefix("Input some text [[fg=green]")
        .with_suffix("[/fg]]")
        .with_filter(filter.clone())
        .with_width(10, 10);

    let mut test_dialog = Dialog::new(30, 4, 4).with_text(
        "Hello! This is some [fg=green][shake=0.5]spooky stuff[/shake][/fg]. I can even add stuff with [bg=red]red background![/bg].",
    );

    let mut parser = Parser::new();
    parser.add_color("green", [0.2, 1.0, 0.2, 1.0]);
    parser.add_color("red", [1.0, 0.2, 0.2, 1.0]);

    let mut menu = Menu::new()
        .with_pos((5, 5))
        .with_focus(true)
        .with_text_processor(parser);

    while terminal.refresh() {
        let events = terminal.get_current_events();

        let dirty = menu.update(
            &events,
            terminal.delta_time(),
            &text_buffer,
            &mut MenuList::new()
                .with_item(&mut title, None)
                .with_item(&mut button, MenuPosition::RelativeToLast(0, 1))
                .with_item(&mut test_input, None)
                .with_item(&mut test_dialog, MenuPosition::RelativeToLast(0, 1)),
        );

        if button.was_just_pressed() {
            times_pressed += 1;
            button.set_text(format!("Times pressed: [fg=red]{}[/fg]", times_pressed));
        }

        if dirty {
            text_buffer.clear();
            menu.draw(&mut text_buffer);
            terminal.flush(&mut text_buffer);
        }

        terminal.draw(&text_buffer);
    }
}
