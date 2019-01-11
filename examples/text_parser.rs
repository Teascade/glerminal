use glerminal::{Parser, TerminalBuilder, TextBuffer};

fn main() {
    let terminal = TerminalBuilder::new()
        .with_title("Parser test")
        .with_dimensions((1280, 720))
        .build();
    let mut text_buffer;
    match TextBuffer::new(&terminal, (80, 24)) {
        Ok(buffer) => text_buffer = buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }

    let mut parser = Parser::new();
    parser.add_color("red", [1.0, 0.2, 0.2, 1.0]);
    parser.add_color("green", [0.2, 1.0, 0.2, 1.0]);
    parser.write(
        &mut text_buffer,
        "Hello, this text is [bg=green][fg=red]red on green[/fg][/bg]! And this text is [shake=0.7][fg=green]shaking[/fg][/shake]!",
    );

    terminal.flush(&mut text_buffer);

    while terminal.refresh() {
        terminal.draw(&text_buffer);
    }
}
