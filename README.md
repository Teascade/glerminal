# OpenGLerminal, an OpenGL terminal
A lightweight terminal made with OpenGL from the ground-up.  
With this terminal, you're able to make the terminal applications or games you've always wanted, but with a terminal that looks the same for everyone, because it's made with OpenGL and doesn't use the computer's native terminal!

Currently supported features include:
- Moving the cursor within the Terminal
- Changing foreground and background colors to whatever you want!
- Shaking text
- A text-parser that will make it easy to write whatever you want and make it look cool!
  - Parseable text example: `"Hello, [fg=red]this is red[/fg] and [shake=1.0]this is shaking[/shake]."`

### How to use
Extensive documentation can be found at: [docs.rs][docs].

Just add the following line to your `Cargo.toml`:
```toml
[dependencies]
glerminal = "0.1.0"
```

And simply add the following line to your `main.rs`:
```rust
extern crate glerminal;
```

And an example of creating a window is simple:
```rust
extern crate glerminal;

use glerminal::terminal::TerminalBuilder;
use glerminal::text_buffer::TextBuffer;

fn main() {
    let terminal = TerminalBuilder::new()
        .with_title("Hello GLerminal!")
        .with_dimensions((1280, 720))
        .build();
    let text_buffer;
    match TextBuffer::new(&terminal, (80, 24)) {
        Ok(buffer) => text_buffer = buffer,
        Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    }

      text_buffer.write("Hello, GLerminal!");
      terminal.flush(&mut text_buffer);

    while terminal.refresh() {
        terminal.draw(&text_buffer);
    }
}
```

### License
This crate is distributed under the terms of [the MIT License][license].

[docs]: https://docs.rs/glerminal
[license]: LICENSE.md
