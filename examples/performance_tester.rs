use glerminal::{TerminalBuilder, TextBuffer, TextStyle};

fn main() {
    let terminal = TerminalBuilder::new().build();
    let mut text_buffer;
    match TextBuffer::create(&terminal, (80, 24)) {
        Ok(buffer) => text_buffer = buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }

    for _ in 0..1920 {
        text_buffer.cursor.style = TextStyle {
            fg_color: [1.0, 0.0, 0.0, 1.0],
            bg_color: [0.0, 1.0, 0.0, 1.0],
            ..Default::default()
        };
        text_buffer.put_char('a');
    }

    let mut frames = 0;
    let mut timer = 0.0;
    let fps_update = 1.0 / 1.0;

    while terminal.refresh() {
        timer += terminal.delta_time();
        frames += 1;
        if timer > fps_update {
            text_buffer.cursor.move_to(0, 0);
            text_buffer.write(format!("{:.6}", frames as f32 / fps_update));
            timer -= fps_update;
            frames = 0;
        }

        terminal.draw(&text_buffer);
    }
}
