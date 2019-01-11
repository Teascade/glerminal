use glerminal::{TerminalBuilder, TextBuffer};

fn main() {
    let terminal = TerminalBuilder::new().build();
    let mut text_buffer;
    match TextBuffer::new(&terminal, (80, 24)) {
        Ok(buffer) => text_buffer = buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }

    for _ in 0..1920 {
        text_buffer.change_cursor_fg_color([1.0, 0.0, 0.0, 1.0]);
        text_buffer.change_cursor_bg_color([0.0, 1.0, 0.0, 1.0]);
        text_buffer.change_cursor_shakiness(0.0);
        text_buffer.put_char('a');
    }

    let mut frames = 0;
    let mut timer = 0.0;
    let fps_update = 1.0 / 20.0;

    while terminal.refresh() {
        timer += terminal.delta_time();
        frames += 1;
        if timer > fps_update {
            text_buffer.move_cursor(0, 0);
            text_buffer.write(format!("{:.6}", frames as f32 / fps_update));
            timer -= fps_update;
            frames = 0;

            terminal.flush(&mut text_buffer);
        }

        terminal.draw(&text_buffer);
    }
}
