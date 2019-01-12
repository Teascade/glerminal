use super::{random_text, run_multiple_times, test_setup_text_buffer};
use crate::menu_systems::{FocusSelection, GrowthDirection, Menu, MenuList, TextItem};
use crate::{Events, VirtualKeyCode};

use rand::{thread_rng, Rng};

#[test]
fn input_selecting() {
    run_multiple_times(20, || {
        let mut rng = thread_rng();

        let mut menu = Menu::new();
        let text_buffer = test_setup_text_buffer((1, 1));

        let mut item1 = TextItem::new("").with_is_button(true);
        let mut item2 = TextItem::new("").with_is_button(true);
        let mut item3 = TextItem::new("");
        let mut item4 = TextItem::new("").with_is_button(true);

        let mut events = Events::new(false);

        let direction = match rng.gen_range(0, 4) {
            0 => GrowthDirection::Down,
            1 => GrowthDirection::Right,
            2 => GrowthDirection::Left,
            _ => GrowthDirection::Up,
        };

        menu.set_growth_direction(direction);
        menu.set_focused(true);

        let change_next = rng.gen();
        let next_key = if change_next {
            VirtualKeyCode::A
        } else {
            match direction {
                GrowthDirection::Down => VirtualKeyCode::Down,
                GrowthDirection::Up => VirtualKeyCode::Up,
                GrowthDirection::Left => VirtualKeyCode::Left,
                GrowthDirection::Right => VirtualKeyCode::Right,
            }
        };

        let change_prev = rng.gen();
        let prev_key = if change_prev {
            VirtualKeyCode::O
        } else {
            match direction {
                GrowthDirection::Down => VirtualKeyCode::Up,
                GrowthDirection::Up => VirtualKeyCode::Down,
                GrowthDirection::Left => VirtualKeyCode::Right,
                GrowthDirection::Right => VirtualKeyCode::Left,
            }
        };

        let next_opt = if change_next { Some(next_key) } else { None };
        let prev_opt = if change_prev { Some(prev_key) } else { None };
        menu.set_focus_selection(FocusSelection::Keyboard(prev_opt, next_opt));

        assert_eq!(menu.get_select_idx(), 0);

        let mut check = |supposed: u32, key: VirtualKeyCode| {
            events.keyboard.update_button_press(key, true);
            menu.update(
                &events,
                0.0,
                &text_buffer,
                &mut MenuList::new()
                    .with_item(&mut item1, None)
                    .with_item(&mut item2, None)
                    .with_item(&mut item3, None)
                    .with_item(&mut item4, None),
            );
            assert_eq!(menu.get_select_idx(), supposed);
            events.keyboard.update_button_press(key, false);
            events.keyboard.clear_just_lists();
        };

        check(1, next_key);
        check(3, next_key);
        check(1, prev_key);
        check(0, prev_key);
        check(3, prev_key);
        check(0, next_key);
    });
}

#[test]
fn draw() {
    run_multiple_times(20, || {
        let mut rng = thread_rng();

        let mut text_buffer = test_setup_text_buffer((8, 4));
        let mut menu = Menu::new();
        let events = Events::new(false);

        let direction = match rng.gen_range(0, 4) {
            0 => GrowthDirection::Down,
            1 => GrowthDirection::Right,
            2 => GrowthDirection::Left,
            _ => GrowthDirection::Up,
        };

        menu.set_growth_direction(direction);
        menu.set_pos((4, 2));
        menu.set_focused(true);

        let text1 = random_text(2);
        let text2 = random_text(2);
        let mut item1 = TextItem::new(text1.clone());
        let mut item2 = TextItem::new(text2.clone());

        let mut up_1 = String::new();
        let mut up_2 = String::new();
        let mut left_1 = String::new();
        let mut left_2 = String::new();
        let mut down_right_1 = String::new();
        let mut down_2 = String::new();
        let mut right_2 = String::new();

        match direction {
            GrowthDirection::Down => {
                down_right_1 = text1;
                down_2 = text2;
            }
            GrowthDirection::Right => {
                down_right_1 = text1;
                right_2 = text2;
            }
            GrowthDirection::Left => {
                left_1 = text1;
                left_2 = text2;
            }
            GrowthDirection::Up => {
                up_1 = text1;
                up_2 = text2;
            }
        }

        let expected_rows = [
            format!("    {up2:>2}  ", up2 = up_2),
            format!("    {up1:>2}  ", up1 = up_1),
            format!(
                "{l2:>2}{l1:>2}{dr1:>2}{r2:>2}",
                l1 = left_1,
                l2 = left_2,
                dr1 = down_right_1,
                r2 = right_2
            ),
            format!("    {d2:>2}  ", d2 = down_2,),
        ];
        // Make a mock text_buffer that looks like this
        // "    u2  "
        // "    u1  "
        // "l2l1d1r2"
        // "    d2  "

        menu.update(
            &events,
            0.0,
            &text_buffer,
            &mut MenuList::new()
                .with_item(&mut item1, None)
                .with_item(&mut item2, None),
        );

        menu.draw(&mut text_buffer);

        let mut actual_rows = [String::new(), String::new(), String::new(), String::new()];
        for (y, row) in actual_rows.iter_mut().enumerate() {
            for x in 0..8 {
                row.push(text_buffer.get_character(x, y as i32).unwrap().get_char());
            }
        }

        assert_eq!(expected_rows, actual_rows);
    });
}
