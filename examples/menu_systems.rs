use glerminal::menu_systems::{
    Checkbox, CheckboxGroup, Dialog, Filter, FocusSelection, GrowthDirection, Menu, MenuList,
    MenuPosition, TextInput, TextItem, Window,
};
use glerminal::{MouseButton, Parser, TerminalBuilder, TextBuffer, TextStyle, VirtualKeyCode};

fn main() {
    let terminal = TerminalBuilder::new()
        .with_title("Simple window")
        .with_dimensions((1280, 720))
        .with_vsync(false)
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

    let mut text_label = TextItem::new("FPS: [fg=green]-[/fg]").with_max_width(40);

    let mut text_input = TextInput::new(None, None)
        .with_prefix("Test your might: ")
        .with_filter(filter.clone())
        .with_character_limit(10)
        .with_focused_colors(([0.2, 0.2, 0.2, 1.0], [0.2, 0.8, 0.2, 1.0]));

    let mut text_input_2 = TextInput::new(10, 10)
        .with_prefix("Test 2: [[fg=green]")
        .with_suffix("[/fg]]")
        .with_filter(filter.clone())
        .with_focused_colors(([0.8, 0.8, 0.8, 1.0], [0.8, 0.2, 0.2, 1.0]))
        .with_caret(0.0);

    let mut checkbox = Checkbox::new("[fg=green]Thing[/fg] 1: ")
        .with_mouse_button_press_inputs(vec![MouseButton::Left]);
    let mut checkbox_2 =
        Checkbox::new("Thing 2: ").with_mouse_button_press_inputs(vec![MouseButton::Left]);
    let mut checkbox_3 =
        Checkbox::new("Thing 3: ").with_mouse_button_press_inputs(vec![MouseButton::Left]);

    let mut button = TextItem::new("Test button!")
        .with_is_button(true)
        .with_mouse_button_press_inputs(vec![MouseButton::Left]);

    let mut parser = Parser::new();
    parser.add_color("green", [0.2, 1.0, 0.2, 1.0]);

    let mut menu = Menu::new()
        .with_pos((5, 5))
        .with_focus(true)
        .with_growth_direction(GrowthDirection::Down)
        .with_focus_selection(FocusSelection::MouseAndKeyboard(None, None))
        .with_text_processor(parser);

    let mut checkbox_group = CheckboxGroup::new().with_force_one_checked(Some(0));

    let mut dialog = Dialog::new(30, 3, 3).with_text(
        "Hello! This is a test text with some stuff, this is supposed to test the dialog window. Testwordlongerthan30charactersfortest",
    ).with_up_buttons(vec!(VirtualKeyCode::O)).with_down_buttons(vec!(VirtualKeyCode::L));

    let test_window = Window::new(70, 20)
        .with_pos((1, 1))
        .with_title("Hello, World!");
    test_window.set_limits(&mut text_buffer);

    let mut frames = 0;
    let mut timer = 0.0;
    let mut button_presses = 0;

    while terminal.refresh() {
        timer += terminal.delta_time();
        frames += 1;
        if timer > 1.0 {
            timer -= 1.0;
            text_label.set_text(format!(
                "FPS: [fg=green]{}[/fg], delta_time (ms): [fg=green]{:.6}[/fg]",
                frames,
                terminal.delta_time() * 1000 as f32
            ));
            frames = 0;
        }

        let events = terminal.get_current_events();

        let mut dirty = menu.update(
            &events,
            terminal.delta_time(),
            &text_buffer,
            &mut MenuList::new()
                .with_item(&mut text_label, None)
                .with_item(&mut text_input, MenuPosition::RelativeToLast(0, 1))
                .with_item(&mut text_input_2, None)
                .with_item(&mut checkbox, None)
                .with_item(&mut checkbox_2, None)
                .with_item(&mut checkbox_3, None)
                .with_item(&mut button, None)
                .with_item(&mut dialog, MenuPosition::RelativeToLast(0, 1)),
        );

        checkbox_group.update(&mut [&mut checkbox, &mut checkbox_2, &mut checkbox_3]);

        if button.was_just_pressed() || text_input_2.was_just_pressed() {
            button_presses += 1;
            dirty = true;
        }

        if dirty {
            text_buffer.clear();

            test_window.draw(&mut text_buffer);
            menu.draw(&mut text_buffer);

            text_buffer.cursor.style = TextStyle {
                fg_color: [0.8, 0.8, 0.8, 1.0],
                bg_color: [0.0, 0.0, 0.0, 0.0],
                ..Default::default()
            };
            text_buffer.cursor.move_to(30, 15);
            text_buffer.write(format!(
                "Text: {} {}",
                text_input.get_text(),
                text_input_2.get_text()
            ));
            text_buffer.cursor.move_to(40, 16);
            text_buffer.write(format!("Button presses: {}", button_presses));
            text_buffer.cursor.move_to(40, 17);
            text_buffer.write(format!("Checked: {:?}", checkbox_group.get_selection_idx()));

            terminal.flush(&mut text_buffer);
        }

        terminal.draw(&text_buffer);
    }
}
