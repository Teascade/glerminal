use std::cell::Cell;

use font::Font;
use text_buffer::text_buffer::TextBuffer;
use super::renderer::{self, Program, Renderable, Texture, Vao, Vbo};

pub struct TextBufferMesh {
    vao: Vao,
    vbo_pos: Vbo,
    vbo_col: Vbo,
    vbo_tex: Vbo,
    vbo_shakiness: Vbo,
    count: Cell<i32>,
    texture: Texture,
}

impl Renderable for TextBufferMesh {
    fn get_vao(&self) -> Vao {
        self.vao
    }

    fn get_count(&self) -> i32 {
        self.count.get()
    }

    fn get_texture(&self) -> Option<Texture> {
        Some(self.texture)
    }
}

impl TextBufferMesh {
    pub fn new(program: Program, dimensions: (i32, i32), font: &Font) -> TextBufferMesh {
        let (width, height) = dimensions;

        let vertex_buffer_pos = vec![0.0; (width * height * 12) as usize];
        let vertex_buffer_col = vec![1.0; (width * height * 24) as usize];
        let vertex_buffer_tex = vec![0.0; (width * height * 12) as usize];
        let vertex_buffer_shakiness = vec![0.0; (width * height * 6) as usize];

        let vbo_pos = renderer::create_vbo(vertex_buffer_pos);
        let vbo_col = renderer::create_vbo(vertex_buffer_col);
        let vbo_tex = renderer::create_vbo(vertex_buffer_tex);
        let vbo_shakiness = renderer::create_vbo(vertex_buffer_shakiness);
        let vao = renderer::create_vao(program, vbo_pos, vbo_col, Some((vbo_tex, vbo_shakiness)));

        let tex = renderer::create_texture(&font.image_buffer, font.width, font.height);
        TextBufferMesh {
            vao: vao,
            vbo_pos: vbo_pos,
            vbo_col: vbo_col,
            vbo_tex: vbo_tex,
            vbo_shakiness: vbo_shakiness,
            count: Cell::new(width * height * 6),
            texture: tex,
        }
    }

    pub fn update(&self, text_buffer: &TextBuffer, font: &Font) {
        if (text_buffer.height * text_buffer.width) as usize != text_buffer.chars.len() {
            panic!("Given TextBuffer height/width do not math chars.len()");
        }

        // Create new position veretex buffer
        let mut vertex_buffer_pos: Vec<f32> = Vec::new();

        // Create new color vertex buffer
        let mut vertex_buffer_col: Vec<f32> = Vec::new();

        // Create new tex coords
        let mut vertex_buffer_tex: Vec<f32> = Vec::new();

        // Create new tex coords
        let mut vertex_buffer_shakiness: Vec<f32> = Vec::new();

        // Fill those arrays
        let character_width = 1.0 / text_buffer.width as f32;
        let character_height = 1.0 / text_buffer.height as f32;
        for y in 0..text_buffer.height {
            for x in 0..text_buffer.width {
                // Calculate pos vertex coords
                let character = text_buffer.get_character(x, y);
                if character.get_char() == ' ' {
                    continue;
                }
                let char_data = match font.get_character(character.get_char()) {
                    Ok(data) => data,
                    Err(error) => panic!(error),
                };
                let width = character_width * (char_data.width as f32 / font.size as f32);
                let height = character_height * (char_data.height as f32 / font.line_height as f32);

                let bmoffset_x =
                    character_width * (char_data.x_off as i32 as f32 / font.size as f32);
                let bmoffset_y = character_height
                    * ((char_data.y_off as i32 - font.max_offset_x as i32) as f32
                        / font.line_height as f32);

                let x_off = x as f32 * character_width + bmoffset_x;
                let y_off = y as f32 * character_height + bmoffset_y;
                let mut single_character_vbuff = vec![
                    x_off,
                    y_off + height,
                    x_off + width,
                    y_off + height,
                    x_off,
                    y_off,
                    x_off + width,
                    y_off,
                    x_off,
                    y_off,
                    x_off + width,
                    y_off + height,
                ];
                vertex_buffer_pos.append(&mut single_character_vbuff);

                // Color and Shakiness
                for _ in 0..6 {
                    vertex_buffer_col.append(&mut character.get_fg_color().to_vec());
                    vertex_buffer_shakiness.push(character.get_shakiness());
                }

                // Calculate tex coords
                let mut char_tex_coords = vec![
                    char_data.x1,
                    char_data.y2,
                    char_data.x2,
                    char_data.y2,
                    char_data.x1,
                    char_data.y1,
                    char_data.x2,
                    char_data.y1,
                    char_data.x1,
                    char_data.y1,
                    char_data.x2,
                    char_data.y2,
                ];

                vertex_buffer_tex.append(&mut char_tex_coords);
            }
        }

        self.count.set((vertex_buffer_pos.len() * 6) as i32);

        renderer::upload_buffer(self.vbo_pos, vertex_buffer_pos);
        renderer::upload_buffer(self.vbo_col, vertex_buffer_col);
        renderer::upload_buffer(self.vbo_tex, vertex_buffer_tex);
        renderer::upload_buffer(self.vbo_shakiness, vertex_buffer_shakiness);
    }
}
