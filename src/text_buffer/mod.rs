#[cfg(feature = "parser")]
pub mod parser;

use font::Font;
use renderer::backgroundmesh::BackgroundMesh;
use renderer::textbuffermesh::TextBufferMesh;
use terminal::Terminal;

/// Represents a color with values from 0.0 to 1.0 (red, green, blue, alpha)
pub type Color = [f32; 4];

/// Represents a raw encoded character.
pub type RawCharacter = u16;

/// The `TextBuffer` acts as a "state machine" where you can set foreground color, background color and shakiness for the cursor,
/// move the cursor around, clear the screen and write with the cursor (using the cursor's styles).
/// It's often the most efficient way to write things, especially if you have a very structured way of displaying things, but for a more simple-to-use
/// way of writing, that isn't as structured ie. for a dialogue, you might want to use the Parser.
///
/// `Parser` is a struct added as a default feature, that is able to take in a piece of text and then parse it and change the cursor styles easily using the `TextBuffer`.
/// The `Parser` can handle tags imilar to BBCode tags, and can change fg, bg and shake, meaning the following tags are available to use mid-text:
/// - `[fg=color]`
/// - `[bg=color]`
/// - `[shake=decimal]`
/// - optional closing/style-resetting tags: `[/fg]`, `[/bg]` and `[/shake]`
///
/// The colors the `Parser` uses mid text must be pre-defined however with `add_color`.
///
/// ### Example usage of `TextBuffer`:
/// ```no_run
/// use glerminal::{TerminalBuilder, TextBuffer};
///
/// let terminal = TerminalBuilder::new()
///     .with_title("Hello GLerminal!")
///     .with_dimensions((1280, 720))
///     .build();
///
/// let mut text_buffer;
/// match TextBuffer::new(&terminal, (80, 24)) {
///   Ok(buffer) => text_buffer = buffer,
///   Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
/// }
///
/// // Test TextBuffer
/// text_buffer.change_cursor_fg_color([1.0, 0.0, 0.0, 1.0]);
/// text_buffer.change_cursor_bg_color([1.0, 1.0, 1.0, 1.0]);
/// text_buffer.change_cursor_shakiness(0.5);
/// text_buffer.move_cursor(0, 0);
/// text_buffer.write("This text is shaking in red in a white background!");
///
/// // Flush to "apply changes"
/// terminal.flush(&mut text_buffer);
/// ```
///
/// ### Example usage of `Parser`
/// ```no_run
/// use glerminal::{TerminalBuilder, TextBuffer, Parser};
///
/// let terminal = TerminalBuilder::new()
///     .with_title("Hello GLerminal!")
///     .with_dimensions((1280, 720))
///     .build();
///
/// let mut text_buffer;
/// match TextBuffer::new(&terminal, (80, 24)) {
///   Ok(buffer) => text_buffer = buffer,
///   Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
/// }
///
/// // Test Parser
/// let mut parser = Parser::new();
/// parser.add_color("red", [1.0, 0.0, 0.0, 1.0]);
/// parser.add_color("white", [1.0, 1.0, 1.0, 1.0]);
/// parser.write(&mut text_buffer, "[fg=red][bg=white][shake=1.0]This text is also shaking in red in a white background![/fg][/bg][/shake]");
/// // Note: it is not necessary to close fg/bg/shake tags, parser will automatically revert colors in the TextBuffer.
///
/// // Flush to "apply changes"
/// terminal.flush(&mut text_buffer);
/// ```
pub struct TextBuffer {
    pub(crate) chars: Vec<TermCharacter>,
    pub(crate) height: i32,
    pub(crate) width: i32,
    pub(crate) mesh: Option<TextBufferMesh>,
    pub(crate) background_mesh: Option<BackgroundMesh>,
    pub(crate) aspect_ratio: f32,
    cursor: TermCursor,

    limits: TermLimits,
    dirty: bool,
}

impl TextBuffer {
    /// Creates a new text buffer with the given dimensions (width in characters, height in characters)
    pub fn new(terminal: &Terminal, dimensions: (i32, i32)) -> Result<TextBuffer, String> {
        let (width, height) = dimensions;

        if width <= 0 || height <= 0 {
            return Err(
                "TextBuffer dimensions are erronous; either width or height is below 1".to_owned(),
            );
        }

        let chars = vec![
            TermCharacter::new(' ' as u16, [0.0; 4], [0.0; 4], 0.0);
            (width * height) as usize
        ];
        let mesh;
        let background_mesh;
        if terminal.headless {
            mesh = None;
            background_mesh = None;
        } else {
            mesh = Some(TextBufferMesh::new(
                terminal.get_program(),
                dimensions,
                &terminal.font,
            ));
            background_mesh = Some(BackgroundMesh::new(
                terminal.get_background_program(),
                dimensions,
            ));
        }

        let true_height = height * terminal.font.line_height as i32;
        let true_width = width * terminal.font.size as i32;

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
            limits: TermLimits::new(width as u32, height as u32),
            aspect_ratio: true_width as f32 / true_height as f32,

            dirty: true,
        })
    }

    pub(crate) fn swap_buffers(&mut self, font: &Font) {
        if self.dirty {
            if let (&Some(ref mesh), &Some(ref background_mesh)) =
                (&self.mesh, &self.background_mesh)
            {
                mesh.update(&self, font);
                background_mesh.update(&self);
            }
            self.dirty = false;
        }
    }

    /// Sets the character at the specified position. It is the user's responsibility to check if such a position exists.
    pub fn set_char(&mut self, x: u32, y: u32, character: TermCharacter) {
        self.chars[(y * self.width as u32 + x) as usize] = character;
    }

    /// Gets the TermChaacter in the given position
    pub fn get_character(&self, x: i32, y: i32) -> TermCharacter {
        self.chars[(y * self.width + x) as usize]
    }

    /// Clears the screen (makes every character empty and resets their style)
    pub fn clear(&mut self) {
        self.chars = vec![
            TermCharacter::new(' ' as u16, [0.0; 4], [0.0; 4], 0.0);
            (self.width * self.height) as usize
        ];
    }

    /// Puts a regular character to the current position of the cursor with the cursor's style
    pub fn put_char(&mut self, character: char) {
        if character.len_utf16() > 1 {
            panic!("Can not insert over 16-bit characters");
        } else {
            let mut bytes = [0; 1];
            character.encode_utf16(&mut bytes);

            self.put_raw_char(bytes[0]);
        }
    }

    /// Puts a raw 16-bit character to the current position of the cursor with the cursor's style
    pub fn put_raw_char(&mut self, character: RawCharacter) {
        self.chars[(self.cursor.y * self.width + self.cursor.x) as usize] = TermCharacter::new(
            character,
            self.cursor.foreground_color,
            self.cursor.background_color,
            self.cursor.shakiness,
        );
        self.move_cursor_by(1);

        self.dirty = true;
    }

    /// Puts the given text the same way as put_char
    pub fn write<T: Into<String>>(&mut self, text: T) {
        let text = text.into();
        for c in text.to_owned().encode_utf16() {
            self.put_raw_char(c);
        }
    }

    /// Changes the foreground color for the cursor
    pub fn change_cursor_fg_color(&mut self, color: Color) {
        self.cursor.foreground_color = color;
    }

    /// Changes the background color of the cursor
    pub fn change_cursor_bg_color(&mut self, color: Color) {
        self.cursor.background_color = color;
    }

    /// Returns the current foreground color of the cursor
    pub fn get_cursor_fg_color(&mut self) -> Color {
        self.cursor.foreground_color
    }

    /// Returns the current background color of the cursor
    pub fn get_cursor_bg_color(&mut self) -> Color {
        self.cursor.background_color
    }

    /// Changes the shakiness of the cursor
    pub fn change_cursor_shakiness(&mut self, shakiness: f32) {
        self.cursor.shakiness = shakiness;
    }

    /// Gets the current shakiness of the cursor
    pub fn get_cursor_shakiness(&mut self) -> f32 {
        self.cursor.shakiness
    }

    /// Moves the cursor to a specified location in the terminal. If the location does not exist, nothing happens.
    pub fn move_cursor(&mut self, x: i32, y: i32) {
        let x = x
            .max(self.limits.get_min_x() as i32)
            .min(self.limits.get_max_x() as i32 - 1);
        let y = y
            .max(self.limits.get_min_y() as i32)
            .min(self.limits.get_max_y() as i32 - 1);
        self.cursor.x = x;
        self.cursor.y = y;
    }

    /// Returns the current position of the cursor
    pub fn get_cursor_position(&self) -> (i32, i32) {
        (self.cursor.x, self.cursor.y)
    }

    /// Set the limits for drawing, other than the current screen.
    /// None means no limit in this direction.
    pub fn set_limits(
        &mut self,
        x_min: Option<u32>,
        x_max: Option<u32>,
        y_min: Option<u32>,
        y_max: Option<u32>,
    ) {
        self.limits.x_min = x_min;
        self.limits.x_max = x_max;
        self.limits.y_min = y_min;
        self.limits.y_max = y_max;
    }

    /// Get the current term limits of the terminal.
    pub fn get_limits(&self) -> TermLimits {
        self.limits.clone()
    }

    /// Returns whether the TextBuffer is dirty or not (whether flush will have any effect or not)
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn move_cursor_by(&mut self, amount: i32) {
        let new_pos = self.cursor.x + amount;
        if new_pos >= 0 {
            self.cursor.x += amount;
            if self.cursor.x >= self.limits.get_max_x() as i32 {
                self.cursor.x = self.limits.get_min_x() as i32;
                self.cursor.y += 1;
                if self.cursor.y >= self.limits.get_max_y() as i32 {
                    self.cursor.y = self.limits.get_min_y() as i32;
                }
            }
        }
    }
}

/// Represents a single character in a [`TextBuffer`](struct.TextBuffer.html)
#[derive(Clone, Copy)]
pub struct TermCharacter {
    character: RawCharacter,
    fg_color: Color,
    bg_color: Color,
    shakiness: f32,
}

impl TermCharacter {
    pub(crate) fn new(
        character: RawCharacter,
        fg_color: Color,
        bg_color: Color,
        shakiness: f32,
    ) -> TermCharacter {
        TermCharacter {
            character,
            fg_color,
            bg_color,
            shakiness,
        }
    }

    /// Gets the char in the TermCharacter
    pub fn get_raw_char(&self) -> RawCharacter {
        self.character
    }

    pub fn get_char(&self) -> char {
        String::from_utf16(&[self.character]).unwrap().remove(0)
    }

    /// Gets the foreground Color in the TermCharacter
    pub fn get_fg_color(&self) -> Color {
        self.fg_color
    }

    /// Gets the background Color in the TermCharacter
    pub fn get_bg_color(&self) -> Color {
        self.bg_color
    }

    /// Gets the shakiness of the TermCharacter, where shakiness is a value from 0.0 to 1.0 (other values are accepted, but these are suggested).
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

/// Represents the limits of the terminal.
#[derive(Clone)]
pub struct TermLimits {
    width: u32,
    height: u32,
    x_min: Option<u32>,
    x_max: Option<u32>,
    y_min: Option<u32>,
    y_max: Option<u32>,
}

impl TermLimits {
    fn new(width: u32, height: u32) -> TermLimits {
        TermLimits {
            width: width,
            height: height,
            x_min: None,
            x_max: None,
            y_min: None,
            y_max: None,
        }
    }

    /// Represents the smallest x-coordinate you should be able to write to (e.g. 0).
    pub fn get_min_x(&self) -> u32 {
        if let Some(x_min) = self.x_min {
            x_min.max(0)
        } else {
            0
        }
    }

    /// Represents the largest x-coordinate, where you should not be able to write anymore. (e.g. screen width)
    pub fn get_max_x(&self) -> u32 {
        if let Some(x_max) = self.x_max {
            x_max.min(self.width)
        } else {
            self.width
        }
    }

    /// Represents the smallest y-coordinate, where you should be able to write to (e.g. 0).
    pub fn get_min_y(&self) -> u32 {
        if let Some(y_min) = self.y_min {
            y_min.max(0)
        } else {
            0
        }
    }

    /// Represents the largest y-coordinate, where you should not be able to write anymore. (e.g. screen height)
    pub fn get_max_y(&self) -> u32 {
        if let Some(y_max) = self.y_max {
            y_max.min(self.height)
        } else {
            self.height
        }
    }
}
