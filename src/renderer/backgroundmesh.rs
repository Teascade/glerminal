use super::{Program, Renderable, Texture, Vao, Vbo};
use crate::{TermCharacter, TextBuffer};

pub(crate) struct BackgroundMesh {
    // Segment size in characters
    segment_size: u32,

    vbo_pos: Vbo,
    vbo_col: Vbo,
    vao: Vao,
    count: i32,
}

impl Renderable for BackgroundMesh {
    fn get_vao(&self) -> Vao {
        self.vao
    }

    fn get_count(&self) -> i32 {
        self.count
    }

    fn get_texture(&self) -> Option<Texture> {
        None
    }
}

impl BackgroundMesh {
    pub fn new(program: Program, dimensions: (u32, u32), segment_rows: u32) -> BackgroundMesh {
        let (width, height) = dimensions;

        let vertex_buffer_pos = vec![-1.0; (width * height * 12) as usize];

        let vertex_buffer_col = vec![0.0; (width * height * 24) as usize];
        let vertex_buffer_shakiness = vec![0.0; (width * height * 6) as usize];

        let vbo_pos = super::create_vbo(&vertex_buffer_pos);
        let vbo_col = super::create_vbo(&vertex_buffer_col);
        let vbo_shakiness = super::create_vbo(&vertex_buffer_shakiness);
        let vao = super::create_vao(program, vbo_pos, vbo_col, vbo_shakiness, None);

        let segment_size = segment_rows * width * 6;
        let count = (width * height * 6) as i32;

        BackgroundMesh {
            segment_size,

            vbo_pos: vbo_pos,
            vbo_col: vbo_col,
            vao: vao,
            count: count,
        }
    }

    pub fn update(&self, text_buffer: &TextBuffer, adjacent_dirty_segments: &[Vec<u32>]) {
        if (text_buffer.height * text_buffer.width) as usize != text_buffer.chars.len() {
            panic!("Given TextBuffer height/width do not math chars.len()");
        }

        let character_width = 1.0 / text_buffer.width as f32;
        let character_height = 1.0 / text_buffer.height as f32;

        for adjacent_segments in adjacent_dirty_segments {
            // Create new position veretex buffer
            let mut vb_pos = Vec::new();
            // Create new color vertex buffer
            let mut vb_col = Vec::new();

            for seg in adjacent_segments {
                let seg_off = seg * text_buffer.segment_rows;
                let row_amount = if *seg == (text_buffer.segment_amount - 1) {
                    text_buffer.last_seg_size
                } else {
                    text_buffer.segment_rows
                };
                for y in 0..row_amount {
                    for x in 0..text_buffer.width {
                        let character = text_buffer.get_character(x, y + seg_off).unwrap();
                        BackgroundMesh::append_char_data(
                            character,
                            (x, y + seg_off),
                            (character_width, character_height),
                            &mut vb_pos,
                            &mut vb_col,
                        );
                    }
                }
            }
            super::update_buffer(
                self.vbo_pos,
                adjacent_segments[0] * self.segment_size * 2,
                &vb_pos,
            );
            super::update_buffer(
                self.vbo_col,
                adjacent_segments[0] * self.segment_size * 4,
                &vb_col,
            );
        }
    }

    fn append_char_data(
        character: TermCharacter,
        curr_position: (u32, u32),
        char_dimensions: (f32, f32),
        vb_pos: &mut Vec<f32>,
        vb_col: &mut Vec<f32>,
    ) {
        if character.style.bg_color == [0.0; 4] {
            vb_pos.append(&mut vec![-1.0; 12]);
            vb_col.append(&mut vec![0.0; 24]);
            return;
        }

        // New Vertex Buffers
        let x_off = curr_position.0 as f32 * char_dimensions.0;
        let y_off = curr_position.1 as f32 * char_dimensions.1;
        let mut single_character_vbuff = vec![
            x_off,
            y_off + char_dimensions.1,
            x_off + char_dimensions.0,
            y_off + char_dimensions.1,
            x_off,
            y_off,
            x_off + char_dimensions.0,
            y_off,
            x_off,
            y_off,
            x_off + char_dimensions.0,
            y_off + char_dimensions.1,
        ];
        vb_pos.append(&mut single_character_vbuff);

        // Get colors
        for _ in 0..6 {
            vb_col.append(&mut character.style.bg_color.to_vec());
        }
    }
}
