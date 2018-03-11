use renderer::TextBufferMesh;
use font::Font;
use terminal::Terminal;

struct TermCursor {
    x: i32,
    y: i32,
}

pub struct TextBuffer {
    pub chars: Vec<char>,
    pub height: i32,
    pub width: i32,
    pub mesh: TextBufferMesh,
    cursor: TermCursor,
}

impl TextBuffer {
    pub fn new(terminal: &Terminal, dimensions: (i32, i32)) -> Result<TextBuffer, String> {
        let (width, height) = dimensions;
        let chars = vec![' '; (width * height) as usize];
        match TextBufferMesh::new(terminal.get_program(), dimensions, &terminal.font) {
            Ok(mesh) => Ok(TextBuffer {
                chars,
                height,
                width,
                mesh,
                cursor: TermCursor { x: 0, y: 0 },
            }),
            Err(err) => Err(err),
        }
    }

    pub(crate) fn swap_buffers(&self, font: &Font) {
        self.mesh.update_tex_coords(&self, font);
    }

    pub fn get_character(&self, x: i32, y: i32) -> char {
        self.chars[(y * self.width + x) as usize]
    }

    pub fn put_char(&mut self, character: char) {
        self.chars[(self.cursor.y * self.width + self.cursor.x) as usize] = character;
        self.move_cursor_by(1);
    }

    pub fn write<T: Into<String>>(&mut self, text: T) {
        let text = text.into();
        for c in text.chars() {
            self.put_char(c);
        }
    }

    pub fn move_cursor(&mut self, x: i32, y: i32) {
        if !self.out_of_bounds(x, y) {
            self.cursor = TermCursor { x: x, y: y };
        }
    }

    fn out_of_bounds(&self, x: i32, y: i32) -> bool {
        (x < 0 || y < 0 || x > self.width || y > self.height)
    }

    fn move_cursor_by(&mut self, amount: i32) {
        let new_pos = self.cursor.x + amount;
        if new_pos >= 0 {
            self.cursor.x += amount;
            if self.cursor.x >= self.width {
                self.cursor.x = 0;
                self.cursor.y += 1;
                if self.cursor.y >= self.height {
                    self.cursor.y = 0;
                }
            }
        }
    }
}
