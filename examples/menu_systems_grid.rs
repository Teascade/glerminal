use glerminal::menu_systems::{
    Menu, MenuList, MenuPosition, MenuSelectionMethod, MenuSwitcher, TextItem,
};
use glerminal::{TerminalBuilder, TextBuffer, VirtualKeyCode};

fn main() {
    let terminal = TerminalBuilder::new()
        .with_title("Menu grid example")
        .with_dimensions((1280, 720))
        .build();
    let mut text_buffer;
    match TextBuffer::create(&terminal, (80, 24)) {
        Ok(buffer) => text_buffer = buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }

    let mut title = TextItem::new("Title!");
    let mut btn1 = TextItem::new("Button 1").with_is_button(true);
    let mut btn2 = TextItem::new("Button 2").with_is_button(true);
    let mut btn3 = TextItem::new("Button 3").with_is_button(true);

    let mut menu_1 = Menu::new().with_pos((3, 3));

    let mut btn4 = TextItem::new("Button 4").with_is_button(true);
    let mut btn5 = TextItem::new("Button 5").with_is_button(true);
    let mut btn6 = TextItem::new("Button 6").with_is_button(true);

    let mut menu_2 = Menu::new().with_pos((12, 5));

    let mut btn7 = TextItem::new("Button 7").with_is_button(true);
    let mut btn8 = TextItem::new("Button 8").with_is_button(true);
    let mut btn9 = TextItem::new("Button 9").with_is_button(true);

    let mut menu_3 = Menu::new().with_pos((21, 5));

    let mut menu_switcher = MenuSwitcher::new(VirtualKeyCode::Left, VirtualKeyCode::Right)
        .with_selection_method(MenuSelectionMethod::ByLocation(false, true));

    while terminal.refresh() {
        let events = terminal.get_current_events();
        let delta = terminal.delta_time();

        let dirty = {
            let menu_1_dirty = menu_1.update(
                &events,
                delta,
                &text_buffer,
                &mut MenuList::new()
                    .with_item(&mut title, None)
                    .with_item(&mut btn1, MenuPosition::RelativeToLast(0, 1))
                    .with_item(&mut btn2, None)
                    .with_item(&mut btn3, None),
            );
            let menu_2_dirty = menu_2.update(
                &events,
                delta,
                &text_buffer,
                &mut MenuList::new()
                    .with_item(&mut btn4, None)
                    .with_item(&mut btn5, None)
                    .with_item(&mut btn6, None),
            );
            let menu_3_dirty = menu_3.update(
                &events,
                delta,
                &text_buffer,
                &mut MenuList::new()
                    .with_item(&mut btn7, None)
                    .with_item(&mut btn8, None)
                    .with_item(&mut btn9, None),
            );
            menu_switcher.update(&events, &mut [&mut menu_1, &mut menu_2, &mut menu_3]);

            menu_1_dirty || menu_2_dirty || menu_3_dirty
        };

        if dirty {
            menu_1.draw(&mut text_buffer);
            menu_2.draw(&mut text_buffer);
            menu_3.draw(&mut text_buffer);
            terminal.flush(&mut text_buffer);
        }

        terminal.draw(&text_buffer);
    }
}
