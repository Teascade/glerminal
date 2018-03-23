extern crate glerminal;

use glerminal::terminal::TerminalBuilder;
use glerminal::text_buffer::TextBuffer;
use glerminal::menu_systems::{Button, Checkbox, Filter, Menu, MenuList, TextInput, TextLabel};

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

    let mut text_label = TextLabel::new("FPS: -", 40);
    let mut empty_space = TextLabel::new("", 0);

    let mut text_input = TextInput::new(None, None)
        .with_prefix("Test your might: ")
        .with_filter(filter.clone())
        .with_character_limit(10)
        .with_focused_colors(([0.2, 0.2, 0.2, 1.0], [0.2, 0.8, 0.2, 1.0]));

    let mut text_input_2 = TextInput::new(10, 10)
        .with_prefix("Test 2: [")
        .with_suffix("]")
        .with_filter(filter.clone())
        .with_focused_colors(([0.8, 0.8, 0.8, 1.0], [0.8, 0.2, 0.2, 1.0]))
        .with_caret(0.0);

    let mut checkbox = Checkbox::new("Thingamajigger: ")
        .with_prefix("(")
        .with_suffix(")")
        .with_checked_text("CHECKED");
    let mut button = Button::new("Test button!", 15);

    let mut menu = Menu::new().with_pos((5, 5)).with_focus(true);

    let mut fps = 0.0;
    let mut button_presses = 0;
    let mut checked = checkbox.is_checked();

    while terminal.refresh() {
        let curr_fps = terminal.get_fps();
        if curr_fps != fps {
            fps = curr_fps;
            text_label.set_text(format!(
                "FPS: {}, delta_time (ms): {:.6}",
                fps,
                terminal.delta_time() * 1000 as f32
            ));
        }

        let input = terminal.get_current_input();

        let mut dirty = menu.update(
            &input,
            terminal.delta_time(),
            &mut MenuList::new()
                .with_item(&mut text_label)
                .with_item(&mut empty_space)
                .with_item(&mut text_input)
                .with_item(&mut text_input_2)
                .with_item(&mut checkbox)
                .with_item(&mut button),
        );

        if button.was_just_pressed() || text_input_2.was_just_pressed() {
            button_presses += 1;
            dirty = true;
        }

        if checkbox.is_checked() != checked {
            checked = checkbox.is_checked();
            dirty = true;
        }

        if dirty {
            text_buffer.clear();
            text_buffer.change_cursor_fg_color([0.8, 0.8, 0.8, 1.0]);
            text_buffer.change_cursor_bg_color([0.0, 0.0, 0.0, 0.0]);
            text_buffer.move_cursor(5, 15);
            text_buffer.write(format!(
                "Text: {} {}",
                text_input.get_text(),
                text_input_2.get_text()
            ));
            text_buffer.move_cursor(5, 16);
            text_buffer.write(format!("Button presses: {}", button_presses));
            text_buffer.move_cursor(5, 17);
            text_buffer.write(format!("Checked: {}", checked));
            menu.draw(&mut text_buffer);

            terminal.flush(&mut text_buffer);
        }

        terminal.draw(&text_buffer);
    }
}
