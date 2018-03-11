use super::renderer::{self, Program, Renderable, Texture, Vao};

pub struct BackgroundMesh {
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

        let vbo_pos = renderer::create_vbo(vertex_buffer_pos, false);
        let vbo_col = renderer::create_vbo(vertex_buffer_col, true);
        let vao = renderer::create_vao(program, vbo_pos, vbo_col, None);

        let count = width * height * 6;

        BackgroundMesh {
            vao: vao,
            count: count,
        }
    }
}
