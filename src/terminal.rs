#[allow(unused_imports)]
use glutin::VirtualKeyCode;
use std::cell::Cell;
use std::time::SystemTime;

use display::Display;
use font::Font;
use text_buffer::TextBuffer;
use renderer;
use input::Input;

pub struct TerminalBuilder {
    title: String,
    dimensions: (u32, u32),
    clear_color: (f32, f32, f32, f32),
    font: Font,
}

#[allow(dead_code)]
impl TerminalBuilder {
    pub fn new() -> TerminalBuilder {
        TerminalBuilder {
            title: "Hello, World!".to_owned(),
            dimensions: (1280, 720),
            clear_color: (0.14, 0.19, 0.28, 1.0),
            font: Font::load("fonts/iosevka.sfl"),
        }
    }

    pub fn with_title<T: Into<String>>(self, title: T) -> TerminalBuilder {
        TerminalBuilder {
            title: title.into(),
            dimensions: self.dimensions,
            clear_color: self.clear_color,
            font: self.font,
        }
    }

    pub fn with_dimensions(self, dimensions: (u32, u32)) -> TerminalBuilder {
        TerminalBuilder {
            title: self.title,
            dimensions: dimensions,
            clear_color: self.clear_color,
            font: self.font,
        }
    }

    pub fn with_clear_color(self, clear_color: (f32, f32, f32, f32)) -> TerminalBuilder {
        TerminalBuilder {
            title: self.title,
            dimensions: self.dimensions,
            clear_color: clear_color,
            font: self.font,
        }
    }

    pub fn with_font(self, font: Font) -> TerminalBuilder {
        TerminalBuilder {
            title: self.title,
            dimensions: self.dimensions,
            clear_color: self.clear_color,
            font: font,
        }
    }

    pub fn build(self) -> Terminal {
        Terminal::new(self.title, self.dimensions, self.clear_color, self.font)
    }
}

pub struct Terminal {
    display: Display,
    program: renderer::Program,
    background_program: renderer::Program,
    debug_program: renderer::Program,
    debug: Cell<bool>,
    since_start: SystemTime,
    pub font: Font,
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

    pub fn set_debug(&self, debug: bool) {
        renderer::set_debug(debug);
        self.debug.set(debug);
    }

    #[cfg(debug_assertions)]
    pub fn refresh(&self) -> bool {
        let input = self.get_current_input();
        if input.was_just_pressed(VirtualKeyCode::F3) {
            self.set_debug(!self.debug.get());
        }
        self.display.refresh()
    }

    #[cfg(not(debug_assertions))]
    pub fn refresh(&self) -> bool {
        self.display.refresh()
    }

    pub fn flush(&self, text_buffer: &mut TextBuffer) {
        text_buffer.swap_buffers(&self.font);
    }

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

    pub fn get_current_input(&self) -> Input {
        self.display.get_current_input()
    }

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
}
