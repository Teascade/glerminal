extern crate glerminal;

use glerminal::terminal::TerminalBuilder;
use glerminal::text_buffer::TextBuffer;
use glerminal::menu_systems::{Filter, Menu, MenuList, TextInput, TextLabel};

fn main() {
    let terminal = TerminalBuilder::new()
        .with_title("Simple window")
        .with_dimensions((1280, 720))
        .build();
    let mut text_buffer;
    match TextBuffer::new(&terminal, (80, 24)) {
        Ok(buffer) => text_buffer = buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }

    let filter = Filter::empty_filter()
        .with_basic_latin_characters()
        .with_basic_numerals()
        .with_basic_special_symbols();

    let mut text_label = TextLabel::new("FPS: -", 30);

    let mut text_input = TextInput::new(10)
        .with_prefix("Test: [")
        .with_suffix("]")
        .with_filter(filter.clone())
        .with_focused_colors(([0.2, 0.2, 0.2, 1.0], [0.2, 0.8, 0.2, 1.0]));

    let mut text_label_2 = TextLabel::new("Hello! Text in-between!", 25);

    let mut text_input_2 = TextInput::new(10)
        .with_prefix("Test 2:  [")
        .with_suffix("]")
        .with_filter(filter.clone())
        .with_focused_colors(([0.2, 0.2, 0.2, 1.0], [0.8, 0.2, 0.2, 1.0]));

    let mut menu = Menu::new().with_pos((5, 5)).with_focus(true);

    let mut fps = 0.0;

    while terminal.refresh() {
        let curr_fps = terminal.get_fps();
        if curr_fps != fps {
            fps = curr_fps;
            text_label.set_text(format!("FPS: {}, delta_time: {}", fps, terminal.delta_time()));
        }

        let input = terminal.get_current_input();

        let dirty = menu.update(
            &input,
            terminal.delta_time(),
            &mut MenuList::new()
                .with_item(&mut text_label)
                .with_item(&mut text_input)
                .with_item(&mut text_label_2)
                .with_item(&mut text_input_2),
        );

        if dirty {
            text_buffer.clear();
            text_buffer.change_cursor_fg_color([0.8, 0.8, 0.8, 1.0]);
            text_buffer.change_cursor_bg_color([0.0, 0.0, 0.0, 0.0]);
            text_buffer.move_cursor(5, 10);
            text_buffer.write(format!(
                "Text: {} {}",
                text_input.get_text(),
                text_input_2.get_text()
            ));
            menu.draw(&mut text_buffer);
            terminal.flush(&mut text_buffer);
        }

        terminal.draw(&text_buffer);
    }
}
