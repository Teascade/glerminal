# GLerminal, an OpenGL terminal
[![Build Status](https://travis-ci.org/Teascade/glerminal.svg?branch=0.1.7)](https://travis-ci.org/Teascade/glerminal)
[![Docs](https://docs.rs/glerminal/badge.svg)](https://docs.rs/glerminal)
[![Crates.io](https://img.shields.io/crates/v/glerminal.svg)](https://crates.io/crates/glerminal)


Read our [Code of Conduct](CODE_OF_CONDUCT.md) and join our [Discord server](https://discord.gg/Wg6D2Rk) if you want to chat!

A lightweight terminal made with OpenGL from the ground-up.  
With this terminal, you're able to make the terminal applications or games you've always wanted, but with a terminal that looks the same for everyone, because it's made with OpenGL and doesn't use the computer's native terminal!

Currently supported features include:
- Moving the cursor within the Terminal
- Changing foreground and background colors to whatever you want!
- Shaking text
- A text-parser that will make it easy to write whatever you want and make it look cool!
  - Parseable text example: `"Hello, [fg=red]this is red[/fg] and [shake=1.0]this is shaking[/shake]."
  
***Note: Requires OpenGL 3.1+ support***

### Table of Contents
- [How to use](#how-to-use)
- [Contributing & Code of Conduct](#contributing-&-code-of-conduct)
- [License](#license)

### How to use
Extensive documentation can be found at: [docs.rs][docs].

Just add the following line to your `Cargo.toml`:
```toml
[dependencies]
glerminal = "0.1"
```

And simply add the following line to your `main.rs`:
```rust
extern crate glerminal;
```

And then using this crate is quite simple:
```rust
extern crate glerminal;

use glerminal::terminal::TerminalBuilder;
use glerminal::text_buffer::TextBuffer;

fn main() {
    let terminal = TerminalBuilder::new()
        .with_title("Hello GLerminal!")
        .with_dimensions((1280, 720))
        .build();
    let mut text_buffer;
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

![What the example looks like](images/example_screenshot.png)

### Contributing & Code of Conduct
You are welcome to contribute to this project, but before do review the [Contributing guidelines](CONTRIBUTING.md).

A Code of Conduct can also be found in the repository as [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md), 
please review it before interacting with the community.

### License
This crate is distributed under the terms of [the MIT License][license].  
This crate also uses a font as a default font, called [Iosevka][iosevka], which is distributed under the terms of [SIL OFL Version 1.1][license-iosevka].

[docs]: https://docs.rs/glerminal
[license]: LICENSE.md
[iosevka]: https://github.com/be5invis/Iosevka
[license-iosevka]: LICENSE-IOSEVKA.md
