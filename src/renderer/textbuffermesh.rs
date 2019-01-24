use super::{Program, Renderable, Texture, Vao, Vbo};
use crate::{CharacterData, Font, TermCharacter, TextBuffer};

pub(crate) struct TextBufferMesh {
    // Segment size in characters
    segment_size: u32,

    vao: Vao,
    vbo_pos: Vbo,
    vbo_col: Vbo,
    vbo_tex: Vbo,
    vbo_shakiness: Vbo,
    count: i32,
    texture: Texture,
}

impl Renderable for TextBufferMesh {
    fn get_vao(&self) -> Vao {
        self.vao
    }

    fn get_count(&self) -> i32 {
        self.count
    }

    fn get_texture(&self) -> Option<Texture> {
        Some(self.texture)
    }
}

impl TextBufferMesh {
    pub fn new(
        program: Program,
        dimensions: (u32, u32),
        segment_rows: u32,
        font: &Font,
    ) -> TextBufferMesh {
        let (width, height) = dimensions;

        let vertex_buffer_pos = vec![-1.0; (width * height * 12) as usize];
        let vertex_buffer_col = vec![0.0; (width * height * 24) as usize];
        let vertex_buffer_tex = vec![0.0; (width * height * 12) as usize];
        let vertex_buffer_shakiness = vec![0.0; (width * height * 6) as usize];

        let vbo_pos = super::create_vbo(&vertex_buffer_pos);
        let vbo_col = super::create_vbo(&vertex_buffer_col);
        let vbo_tex = super::create_vbo(&vertex_buffer_tex);
        let vbo_shakiness = super::create_vbo(&vertex_buffer_shakiness);
        let vao = super::create_vao(program, vbo_pos, vbo_col, vbo_shakiness, Some(vbo_tex));

        let tex = super::create_texture(&font.image_buffer, font.width, font.height);

        let segment_size = segment_rows * width * 6;
        let count = (width * height * 6) as i32;

        TextBufferMesh {
            segment_size,

            vao: vao,
            vbo_pos: vbo_pos,
            vbo_col: vbo_col,
            vbo_tex: vbo_tex,
            vbo_shakiness: vbo_shakiness,
            count: count,
            texture: tex,
        }
    }

    pub fn update(
        &self,
        text_buffer: &TextBuffer,
        adjacent_dirty_segments: &[Vec<u32>],
        font: &Font,
    ) {
        if (text_buffer.height * text_buffer.width) as usize != text_buffer.chars.len() {
            panic!("Given TextBuffer height/width do not math chars.len()");
        }

        // Fill those arrays
        let character_width = 1.0 / text_buffer.width as f32;
        let character_height = 1.0 / text_buffer.height as f32;

        let default_char_data = match font.get_character('?' as u16) {
            Ok(data) => data,
            Err(_) => font.get_character(' ' as u16).ok().unwrap(),
        };

        println!("Dirty segments: {:?}", adjacent_dirty_segments);

        for adjacent_segments in adjacent_dirty_segments {
            // Create new position veretex buffer
            let mut vb_pos = Vec::new();
            // Create new color vertex buffer
            let mut vb_col = Vec::new();
            // Create new tex coords
            let mut vb_tex = Vec::new();
            // Create new shakiness buffer
            let mut vb_shakiness = Vec::new();

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
                        TextBufferMesh::append_char_data(
                            (&default_char_data, character, &font),
                            (x, y + seg_off),
                            (character_width, character_height),
                            &mut vb_pos,
                            &mut vb_col,
                            &mut vb_tex,
                            &mut vb_shakiness,
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
                self.vbo_tex,
                adjacent_segments[0] * self.segment_size * 2,
                &vb_tex,
            );
            super::update_buffer(
                self.vbo_col,
                adjacent_segments[0] * self.segment_size * 4,
                &vb_col,
            );
            super::update_buffer(
                self.vbo_shakiness,
                adjacent_segments[0] * self.segment_size,
                &vb_shakiness,
            );
        }
    }

    fn append_char_data(
        // Default data, character and font
        character_datas: (&CharacterData, TermCharacter, &Font),
        curr_position: (u32, u32),
        char_dimensions: (f32, f32),
        vb_pos: &mut Vec<f32>,
        vb_col: &mut Vec<f32>,
        vb_tex: &mut Vec<f32>,
        vb_shakiness: &mut Vec<f32>,
    ) {
        let (default_char_data, character, font) = character_datas;
        if character.get_raw_char() == (' ' as u16) {
            vb_pos.append(&mut vec![-1.0; 12]);
            vb_col.append(&mut vec![0.0; 24]);
            vb_shakiness.append(&mut vec![0.0; 6]);
            vb_tex.append(&mut vec![0.0; 12]);
            return;
        }
        let char_data = match font.get_character(character.get_raw_char()) {
            Ok(data) => data,
            Err(_) => default_char_data.clone(),
        };
        let width = char_dimensions.0 * (char_data.width as f32 / font.average_xadvance as f32);
        let height = char_dimensions.1 * (char_data.height as f32 / font.line_height as f32);

        let bmoffset_x =
            char_dimensions.0 * (char_data.x_off as f32 / font.average_xadvance as f32);
        let bmoffset_y = char_dimensions.1 * (char_data.y_off as f32 / font.line_height as f32);

        let x_off = curr_position.0 as f32 * char_dimensions.0 + bmoffset_x;
        let y_off = curr_position.1 as f32 * char_dimensions.1 + bmoffset_y;
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
        vb_pos.append(&mut single_character_vbuff);

        // Color and Shakiness
        for _ in 0..6 {
            vb_col.append(&mut character.style.fg_color.to_vec());
            vb_shakiness.push(character.style.shakiness);
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

        vb_tex.append(&mut char_tex_coords);
    }
}
