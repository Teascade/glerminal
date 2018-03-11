use renderer::textbuffermesh::TextBufferMesh;
use renderer::backgroundmesh::BackgroundMesh;
use font::Font;
use terminal::Terminal;

type Color = [f32; 4];

#[derive(Clone, Copy)]
pub struct TermCharacter {
    character: char,
    fg_color: Color,
    bg_color: Color,
}

impl TermCharacter {
    pub fn new(character: char, fg_color: Color, bg_color: Color) -> TermCharacter {
        TermCharacter {
            character,
            fg_color,
            bg_color,
        }
    }

    pub fn get_char(&self) -> char {
        self.character
    }

    pub fn get_fg_color(&self) -> Color {
        self.fg_color
    }

    pub fn get_bg_color(&self) -> Color {
        self.bg_color
    }
}

struct TermCursor {
    x: i32,
    y: i32,
}

pub struct TextBuffer {
    pub(crate) chars: Vec<TermCharacter>,
    pub(crate) height: i32,
    pub(crate) width: i32,
    pub(crate) mesh: TextBufferMesh,
    pub(crate) background_mesh: BackgroundMesh,
    cursor: TermCursor,
}

impl TextBuffer {
    pub fn new(terminal: &Terminal, dimensions: (i32, i32)) -> Result<TextBuffer, String> {
        let (width, height) = dimensions;

        if width <= 0 || height <= 0 {
            return Err(
                "TextBuffer dimensions are erronous; either width or height is below 1".to_owned(),
            );
        }

        let chars = vec![TermCharacter::new(' ', [0.0; 4], [0.0; 4]); (width * height) as usize];
        let mesh = TextBufferMesh::new(terminal.get_program(), dimensions, &terminal.font);
        let background_mesh = BackgroundMesh::new(terminal.get_background_program(), dimensions);
        Ok(TextBuffer {
            chars,
            height,
            width,
            mesh,
            background_mesh,
            cursor: TermCursor { x: 0, y: 0 },
        })
    }

    pub(crate) fn swap_buffers(&self, font: &Font) {
        self.mesh.update(&self, font);
        self.background_mesh.update(&self);
    }

    pub fn get_character(&self, x: i32, y: i32) -> TermCharacter {
        self.chars[(y * self.width + x) as usize]
    }

    pub fn put_char(&mut self, character: char, fg_color: Color, bg_color: Color) {
        self.chars[(self.cursor.y * self.width + self.cursor.x) as usize] =
            TermCharacter::new(character, fg_color, bg_color);
        self.move_cursor_by(1);
    }

    pub fn write<T: Into<String>>(&mut self, text: T, fg_color: Color, bg_color: Color) {
        let text = text.into();
        for c in text.chars() {
            self.put_char(c, fg_color, bg_color);
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
