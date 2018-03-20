extern crate glerminal;

use glerminal::terminal::TerminalBuilder;
use glerminal::text_buffer::TextBuffer;
use glerminal::menu_systems::{Filter, Menu, MenuItem, TextInput};

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

    let mut text_input =
        MenuItem::new(TextInput::new(10)
            .with_prefix("Test: [")
            .with_suffix("]")
            .with_filter(filter.clone()));

    let mut text_input_2 =
        MenuItem::new(TextInput::new(10)
            .with_prefix("Test 2:  [")
            .with_suffix("]")
            .with_filter(filter.clone()));

    let mut menu = Menu::new().with_pos((5, 5)).with_focus(true);

    let mut fps = 0.0;

    while terminal.refresh() {
        let mut should_flush = false;

        let curr_fps = terminal.get_fps();
        if curr_fps != fps {
            fps = curr_fps;
            should_flush = true;
        }

        let input = terminal.get_current_input();

        menu.handle_input(
            &input,
            &mut [&mut text_input, &mut text_input_2],
        );

        if menu.is_dirty(&[&text_input, &text_input_2]) {
            should_flush = true;
        }

        if should_flush {
            text_buffer.clear();
            text_buffer.change_cursor_fg_color([0.8, 0.8, 0.8, 1.0]);
            text_buffer.change_cursor_bg_color([0.0, 0.0, 0.0, 0.0]);
            text_buffer.move_cursor(5, 4);
            text_buffer.move_cursor(5, 10);
            text_buffer.write(format!(
                "{}{}",
                text_input.get_text(),
                text_input_2.get_text()
            ));
            menu.draw(
                &mut text_buffer,
                &mut [&mut text_input, &mut text_input_2],
            );
            terminal.flush(&mut text_buffer);
        }

        terminal.draw(&text_buffer);
    }
}
