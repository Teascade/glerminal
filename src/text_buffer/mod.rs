pub mod parser;

use renderer::textbuffermesh::TextBufferMesh;
use renderer::backgroundmesh::BackgroundMesh;
use font::Font;
use terminal::Terminal;

pub type Color = [f32; 4];

#[derive(Clone, Copy)]
pub struct TermCharacter {
    character: char,
    fg_color: Color,
    bg_color: Color,
    shakiness: f32,
}

impl TermCharacter {
    pub fn new(character: char, fg_color: Color, bg_color: Color, shakiness: f32) -> TermCharacter {
        TermCharacter {
            character,
            fg_color,
            bg_color,
            shakiness,
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

    pub fn get_shakiness(&self) -> f32 {
        self.shakiness
    }
}

struct TermCursor {
    x: i32,
    y: i32,
    foreground_color: Color,
    background_color: Color,
    shakiness: f32,
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

        let chars =
            vec![TermCharacter::new(' ', [0.0; 4], [0.0; 4], 0.0); (width * height) as usize];
        let mesh = TextBufferMesh::new(terminal.get_program(), dimensions, &terminal.font);
        let background_mesh = BackgroundMesh::new(terminal.get_background_program(), dimensions);
        Ok(TextBuffer {
            chars,
            height,
            width,
            mesh,
            background_mesh,
            cursor: TermCursor {
                x: 0,
                y: 0,
                foreground_color: [1.0; 4],
                background_color: [0.0; 4],
                shakiness: 0.0,
            },
        })
    }

    pub(crate) fn swap_buffers(&self, font: &Font) {
        self.mesh.update(&self, font);
        self.background_mesh.update(&self);
    }

    pub fn get_character(&self, x: i32, y: i32) -> TermCharacter {
        self.chars[(y * self.width + x) as usize]
    }

    fn out_of_bounds(&self, x: i32, y: i32) -> bool {
        (x < 0 || y < 0 || x > self.width || y > self.height)
    }

    pub fn clear(&mut self) {
        self.chars = vec![
            TermCharacter::new(' ', [0.0; 4], [0.0; 4], 0.0);
            (self.width * self.height) as usize
        ];
    }

    pub fn put_char(&mut self, character: char) {
        self.chars[(self.cursor.y * self.width + self.cursor.x) as usize] = TermCharacter::new(
            character,
            self.cursor.foreground_color,
            self.cursor.background_color,
            self.cursor.shakiness,
        );
        self.move_cursor_by(1);
    }

    pub fn write<T: Into<String>>(&mut self, text: T) {
        let text = text.into();
        for c in text.chars() {
            self.put_char(c);
        }
    }

    pub fn change_cursor_fg_color(&mut self, color: Color) {
        self.cursor.foreground_color = color;
    }

    pub fn change_cursor_bg_color(&mut self, color: Color) {
        self.cursor.background_color = color;
    }

    pub fn get_cursor_fg_color(&mut self) -> Color {
        self.cursor.foreground_color
    }

    pub fn get_cursor_bg_color(&mut self) -> Color {
        self.cursor.background_color
    }

    pub fn change_cursor_shakiness(&mut self, shakiness: f32) {
        self.cursor.shakiness = shakiness;
    }

    pub fn get_cursor_shakiness(&mut self) -> f32 {
        self.cursor.shakiness
    }

    pub fn move_cursor(&mut self, x: i32, y: i32) {
        if !self.out_of_bounds(x, y) {
            self.cursor.x = x;
            self.cursor.y = y;
        }
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
