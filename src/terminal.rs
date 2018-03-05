use display::Display;
use font::Font;
use text_buffer::TextBuffer;
use renderer;

pub type Program = u32;

pub struct Terminal {
    display: Display,
    program: Program,
    pub font: Font,
}

impl Terminal {
    pub fn new(window_dimensions: (u32, u32)) -> Terminal {
        let display = Display::new("Hello, World!", window_dimensions);
        let program = renderer::create_program();
        let font = Font::load(
            "fonts/vcr_osd_mono_regular_48.png",
            "fonts/vcr_osd_mono_regular_48.sfl",
        );
        Terminal {
            display,
            program,
            font,
        }
    }

    pub fn refresh(&self) -> bool {
        self.display.refresh()
    }

    pub fn flush(&self, text_buffer: &TextBuffer) {
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

    pub fn get_program(&self) -> Program {
        self.program
    }

    pub fn set_title<T: Into<String>>(&mut self, title: T) {
        self.display.set_title(&title.into());
    }
}
