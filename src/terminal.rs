//! This module acts as the window and "canvas" of the terminal, handling most behind-the-sceneries
//!
//! The [`Terminal`](struct.Terminal.html) is used to create the window and canvas for the [`TextBuffer`](text_buffer/struct.TextBuffer.html)
//! which can then draw it, close the window, reset the title of the window or handle input.
//!
//! **Note** when building with debug-mode, you are able to press `F3` to toggle between debug and non-debug. see ([`Terminal`](struct.Terminal.html#method.set_debug)) for more information.
//!
//! ### Terminal example:
//! ```
//! use glerminal::terminal::TerminalBuilder;
//!
//! let mut terminal = TerminalBuilder::new()
//!     .with_title("Hello GLerminal!")
//!     .with_dimensions((1280, 720))
//!     .build();
//!
//! terminal.set_title("Changed title!");
//! ```

#[allow(unused_imports)]
use glutin::VirtualKeyCode;
use std::cell::Cell;
use std::time::SystemTime;

use display::Display;
use font::Font;
use text_buffer::TextBuffer;
use renderer;
use input::Input;

/// A builder for the `Terminal`. Includes some settings that can be set before building.
///
/// See [terminal mod](index.html) for examples and more detailed documentation.
pub struct TerminalBuilder {
    title: String,
    dimensions: (u32, u32),
    clear_color: (f32, f32, f32, f32),
    font: Font,
}

#[allow(dead_code)]
impl TerminalBuilder {
    /// Creates a new terminal builder with default settings.
    pub fn new() -> TerminalBuilder {
        TerminalBuilder {
            title: "Hello, World!".to_owned(),
            dimensions: (1280, 720),
            clear_color: (0.14, 0.19, 0.28, 1.0),
            font: Font::load("fonts/iosevka.sfl"),
        }
    }

    /// Sets the title for the `Terminal`.
    pub fn with_title<T: Into<String>>(self, title: T) -> TerminalBuilder {
        TerminalBuilder {
            title: title.into(),
            dimensions: self.dimensions,
            clear_color: self.clear_color,
            font: self.font,
        }
    }

    /// Sets the dimensions the `Terminal` is to be opened with.
    pub fn with_dimensions(self, dimensions: (u32, u32)) -> TerminalBuilder {
        TerminalBuilder {
            title: self.title,
            dimensions: dimensions,
            clear_color: self.clear_color,
            font: self.font,
        }
    }

    /// Sets the clear color of the terminal.
    pub fn with_clear_color(self, clear_color: (f32, f32, f32, f32)) -> TerminalBuilder {
        TerminalBuilder {
            title: self.title,
            dimensions: self.dimensions,
            clear_color: clear_color,
            font: self.font,
        }
    }

    /// Changes the font that the terminal uses.
    pub fn with_font(self, font: Font) -> TerminalBuilder {
        TerminalBuilder {
            title: self.title,
            dimensions: self.dimensions,
            clear_color: self.clear_color,
            font: font,
        }
    }

    /// Builds the actual terminal and opens the window
    pub fn build(self) -> Terminal {
        Terminal::new(self.title, self.dimensions, self.clear_color, self.font)
    }
}

/// Represents the Terminal itself.
///
/// See [terminal mod](index.html) for examples and more detailed documentation.
pub struct Terminal {
    display: Display,
    program: renderer::Program,
    background_program: renderer::Program,
    debug_program: renderer::Program,
    debug: Cell<bool>,
    since_start: SystemTime,
    pub(crate) font: Font,
}

impl Terminal {
    fn new<T: Into<String>>(
        title: T,
        window_dimensions: (u32, u32),
        clear_color: (f32, f32, f32, f32),
        font: Font,
    ) -> Terminal {
        let display = Display::new(title, window_dimensions, clear_color);
        let program = renderer::create_program(renderer::VERT_SHADER, renderer::FRAG_SHADER);
        let background_program =
            renderer::create_program(renderer::VERT_SHADER, renderer::BG_FRAG_SHADER);
        let debug_program =
            renderer::create_program(renderer::VERT_SHADER, renderer::DEBUG_FRAG_SHADER);
        let font = font;
        Terminal {
            display,
            program,
            background_program,
            debug_program,
            debug: Cell::new(false),
            since_start: SystemTime::now(),
            font,
        }
    }

    /// Sets debug mode (changes characters and backgrounds into wireframe)
    pub fn set_debug(&self, debug: bool) {
        renderer::set_debug(debug);
        self.debug.set(debug);
    }

    /// Refreshes the screen and returns weather the while-loop should continue (is the program running)
    #[cfg(debug_assertions)]
    pub fn refresh(&self) -> bool {
        let input = self.get_current_input();
        if input.was_just_pressed(VirtualKeyCode::F3) {
            self.set_debug(!self.debug.get());
        }
        self.display.refresh()
    }

    /// Refreshes the screen and returns weather the while-loop should continue (is the program running)
    #[cfg(not(debug_assertions))]
    pub fn refresh(&self) -> bool {
        self.display.refresh()
    }

    /// Flushes `TextBuffer`, taking it's character-grid and making it show for the next draw.
    ///
    /// This is quite a heavy function and it's calling should be avoided when unnecessary.
    pub fn flush(&self, text_buffer: &mut TextBuffer) {
        text_buffer.swap_buffers(&self.font);
    }

    /// Draws the `TextBuffer`, this should be called every time in the while-loop.
    pub fn draw(&self, text_buffer: &TextBuffer) {
        renderer::clear();
        let duration = SystemTime::now().duration_since(self.since_start).unwrap();

        let time = duration.as_secs() as f32 + duration.subsec_nanos() as f32 / 1_000_000_000.0;

        renderer::draw(
            self.get_background_program(),
            self.display.proj_matrix.get(),
            time,
            &text_buffer.background_mesh,
        );
        renderer::draw(
            self.get_program(),
            self.display.proj_matrix.get(),
            time,
            &text_buffer.mesh,
        );
    }

    /// Gets the current Input, must be retrieved every time you want new inputs. (ie. every frame)
    pub fn get_current_input(&self) -> Input {
        self.display.get_current_input()
    }

    /// Sets the title for the window
    pub fn set_title<T: Into<String>>(&mut self, title: T) {
        self.display.set_title(&title.into());
    }

    pub(crate) fn get_program(&self) -> renderer::Program {
        if !self.debug.get() {
            self.program
        } else {
            self.debug_program
        }
    }

    pub(crate) fn get_background_program(&self) -> renderer::Program {
        if !self.debug.get() {
            self.background_program
        } else {
            self.debug_program
        }
    }

    // TODO: Add .close
}
