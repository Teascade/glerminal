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
    nondebug_program: renderer::Program,
    debug_program: renderer::Program,
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
        let debug_program =
            renderer::create_program(renderer::VERT_SHADER, renderer::DEBUG_FRAG_SHADER);
        let font = font;
        Terminal {
            display,
            program,
            nondebug_program: program,
            debug_program,
            font,
        }
    }

    pub fn set_debug(&mut self, debug: bool) {
        renderer::set_debug(debug);
        if debug {
            self.program = self.debug_program;
        } else {
            self.program = self.nondebug_program;
        }
    }

    pub fn refresh(&self) -> bool {
        self.display.refresh()
    }

    pub fn flush(&self, text_buffer: &mut TextBuffer) {
        text_buffer.swap_buffers(&self.font);
    }

    pub fn draw(&self, text_buffer: &TextBuffer) {
        renderer::clear();
        renderer::draw(
            self.program,
            self.display.proj_matrix.get(),
            &text_buffer.mesh,
        );
    }

    pub fn get_current_input(&self) -> Input {
        self.display.get_current_input()
    }

    pub fn get_program(&self) -> renderer::Program {
        self.program
    }

    pub fn set_title<T: Into<String>>(&mut self, title: T) {
        self.display.set_title(&title.into());
    }
}
