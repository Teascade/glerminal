use std::cell::Cell;

use super::{Program, Renderable, Texture, Vao, Vbo};
use text_buffer::TextBuffer;

pub struct BackgroundMesh {
    width: i32,
    height: i32,
    vbo_pos: Vbo,
    vbo_col: Vbo,
    vao: Vao,
    count: Cell<i32>,
}

impl Renderable for BackgroundMesh {
    fn get_vao(&self) -> Vao {
        self.vao
    }

    fn get_count(&self) -> i32 {
        self.count.get()
    }

    fn get_texture(&self) -> Option<Texture> {
        None
    }
}

impl BackgroundMesh {
    pub fn new(program: Program, dimensions: (i32, i32)) -> BackgroundMesh {
        let (width, height) = dimensions;

        let mut vertex_buffer_pos = Vec::new();

        let character_width = 1.0 / width as f32;
        let character_height = 1.0 / height as f32;
        for y in 0..height {
            for x in 0..width {
                let x_off = x as f32 * character_width;
                let y_off = y as f32 * character_height;
                let mut single_character_vbuff = vec![
                    x_off,
                    y_off + character_height,
                    x_off + character_width,
                    y_off + character_height,
                    x_off,
                    y_off,
                    x_off + character_width,
                    y_off,
                    x_off,
                    y_off,
                    x_off + character_width,
                    y_off + character_height,
                ];
                vertex_buffer_pos.append(&mut single_character_vbuff);
            }
        }

        let vertex_buffer_col = vec![0.0; (width * height * 24) as usize];
        let vertex_buffer_shakiness = vec![0.0; (width * height * 6) as usize];

        let vbo_pos = super::create_vbo(vertex_buffer_pos);
        let vbo_col = super::create_vbo(vertex_buffer_col);
        let vbo_shakiness = super::create_vbo(vertex_buffer_shakiness);
        let vao = super::create_vao(program, vbo_pos, vbo_col, vbo_shakiness, None);

        let count = width * height * 6;

        BackgroundMesh {
            width: width,
            height: height,
            vbo_pos: vbo_pos,
            vbo_col: vbo_col,
            vao: vao,
            count: Cell::new(count),
        }
    }

    pub fn update(&self, text_buffer: &TextBuffer) {
        if (text_buffer.height * text_buffer.width) as usize != text_buffer.chars.len() {
            panic!("Given TextBuffer height/width do not math chars.len()");
        }

        // Create new color vertex buffer
        let mut vertex_buffer_pos: Vec<f32> = Vec::new();

        // Create new color vertex buffer
        let mut vertex_buffer_col: Vec<f32> = Vec::new();

        let character_width = 1.0 / self.width as f32;
        let character_height = 1.0 / self.height as f32;
        for y in 0..text_buffer.height {
            for x in 0..text_buffer.width {
                let character = text_buffer.get_character(x, y);

                if character.get_bg_color() == [0.0; 4] {
                    continue;
                }

                // New Vertex Buffers
                let x_off = x as f32 * character_width;
                let y_off = y as f32 * character_height;
                let mut single_character_vbuff = vec![
                    x_off,
                    y_off + character_height,
                    x_off + character_width,
                    y_off + character_height,
                    x_off,
                    y_off,
                    x_off + character_width,
                    y_off,
                    x_off,
                    y_off,
                    x_off + character_width,
                    y_off + character_height,
                ];
                vertex_buffer_pos.append(&mut single_character_vbuff);

                // Get colors
                for _ in 0..6 {
                    vertex_buffer_col.append(&mut character.get_bg_color().to_vec());
                }
            }
        }

        self.count.set((vertex_buffer_pos.len() * 6) as i32);

        super::upload_buffer(self.vbo_pos, vertex_buffer_pos);
        super::upload_buffer(self.vbo_col, vertex_buffer_col);
    }
}
