use display::Display;
use font::Font;
use text_buffer::TextBuffer;
use renderer;

pub struct Terminal {
    display: Display,
    program: u32,
    font: Font,
    text_buffer: TextBuffer,
}

impl Terminal {
    pub fn new(
        window_dimensions: (u32, u32),
        terminal_dimensions: (i32, i32),
    ) -> Result<Terminal, String> {
        let display = Display::new("Hello, World!", window_dimensions);
        let program = renderer::create_program();
        let font = Font::load(
            "fonts/vcr_osd_mono_regular_48.png",
            "fonts/vcr_osd_mono_regular_48.sfl",
        );
        match TextBuffer::new(program, terminal_dimensions, &font) {
            Ok(text_buffer) => Ok(Terminal {
                display,
                program,
                font,
                text_buffer,
            }),
            Err(error) => Err(error),
        }
    }

    pub fn refresh(&mut self) -> bool {
        self.display.refresh()
    }

    pub fn draw(&self) {
        renderer::clear();
        renderer::draw(
            self.program,
            self.display.proj_matrix,
            &self.text_buffer.mesh,
        );
    }

    pub fn swap_buffers(&self) {
        self.text_buffer.swap_buffers(&self.font);
    }

    pub fn set_title<T: Into<String>>(&mut self, title: T) {
        self.display.set_title(&title.into());
    }

    pub fn write<T: Into<String>>(&mut self, text: T) {
        self.text_buffer.write(text.into());
    }
}
