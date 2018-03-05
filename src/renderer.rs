use gl;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str::from_utf8;
use std::ffi::{CStr, CString};

use font::Font;

static VERT_SHADER: &'static str = include_str!("shaders/vert_shader.glsl");
static FRAG_SHADER: &'static str = include_str!("shaders/frag_shader.glsl");

pub type Matrix4 = [f32; 16];

pub struct Renderable {
    vao: u32,
    count: i32,
    texture: u32,
}

impl Renderable {
    pub fn new_box(program: u32, font: Font, character: char) -> Renderable {
        let vertex_buffer = vec![0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0];
        let char_data;
        match font.get_character(character) {
            Ok(data) => char_data = data,
            Err(error) => panic!(error),
        }
        let tex_coords = vec![
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
        let vbo_pos = create_vbo(vertex_buffer);
        let vbo_tex = create_vbo(tex_coords);
        let vao = create_vao(program, vbo_pos, vbo_tex);

        let tex = create_texture(font.image_buffer, font.width, font.height);
        Renderable {
            vao: vao,
            count: 6,
            texture: tex,
        }
    }
}

pub fn clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn create_program() -> u32 {
    unsafe {
        let vert = create_shader(VERT_SHADER, gl::VERTEX_SHADER);
        let frag = create_shader(FRAG_SHADER, gl::FRAGMENT_SHADER);

        let program = gl::CreateProgram();

        gl::AttachShader(program, vert);
        gl::AttachShader(program, frag);

        gl::LinkProgram(program);

        program
    }
}

pub fn update_viewport(dimensions: (u32, u32)) {
    let (width, height) = dimensions;
    unsafe {
        gl::Viewport(0, 0, width as i32, height as i32);
    }
}

pub fn create_proj_matrix(dimensions: (f32, f32), aspect_ratio: f32) -> Matrix4 {
    let (width, height) = dimensions;
    let true_width = height * aspect_ratio;
    let true_height = width / aspect_ratio;
    let mut overflow_width = 0f32;
    let mut overflow_height = 0f32;
    if true_width < width {
        overflow_width = (width - true_width) / true_width;
    } else {
        overflow_height = (height - true_height) / true_height;
    }
    let left = 0.0 - overflow_width / 2 as f32;
    let top = 0.0 - overflow_height / 2 as f32;
    let right = 1.0 + overflow_width / 2 as f32;
    let bottom = 1.0 + overflow_height / 2 as f32;

    let far = 1.0;
    let near = -1.0;
    [
        2.0 / (right - left),
        0.0,
        0.0,
        -(right + left) / (right - left),
        0.0,
        2.0 / (top - bottom),
        0.0,
        -(top + bottom) / (top - bottom),
        0.0,
        0.0,
        -2.0 / (far - near),
        -(far + near) / (far - near),
        0.0,
        0.0,
        0.0,
        1.0,
    ]
}

pub fn draw(program: u32, proj_matrix: Matrix4, renderable: &Renderable) {
    unsafe {
        gl::UseProgram(program);
        gl::BindTexture(gl::TEXTURE_2D, renderable.texture);
        gl::BindVertexArray(renderable.vao);

        let loc = gl::GetUniformLocation(
            program,
            CString::new("proj_mat".to_string()).unwrap().as_ptr() as *const i8,
        ) as i32;
        gl::UniformMatrix4fv(loc, 1, gl::TRUE, proj_matrix.as_ptr());

        gl::DrawArrays(gl::TRIANGLES, 0, renderable.count);
    }
}

pub fn create_texture(pixels: Vec<u8>, width: u32, height: u32) -> u32 {
    unsafe {
        let mut tex = 0;
        gl::GenTextures(1, &mut tex);
        gl::BindTexture(gl::TEXTURE_2D, tex);

        // filters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        let pixels: &[u8] = pixels.as_slice();
        let data_pointer = pixels.as_ptr() as *const c_void;
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data_pointer,
        );

        tex
    }
}

fn create_vbo(vertex_buffer: Vec<f32>) -> u32 {
    unsafe {
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let vertex_data: &[f32] = vertex_buffer.as_slice();

        let data_length = (vertex_data.len() * mem::size_of::<f32>()) as gl::types::GLsizeiptr;
        let data_pointer = vertex_data.as_ptr() as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, data_length, data_pointer, gl::STATIC_DRAW);
        vbo
    }
}

fn get_attrib_location(program: u32, attribute: &str) -> u32 {
    unsafe {
        gl::GetAttribLocation(
            program,
            CString::new(attribute).unwrap().as_ptr() as *const i8,
        ) as u32
    }
}

fn create_vao(program: u32, vbo_pos: u32, vbo_tex: u32) -> u32 {
    unsafe {
        let mut vao = 0;

        gl::GenVertexArrays(1, &mut vao);

        let attrib_location = get_attrib_location(program, "position");

        gl::BindVertexArray(vao);
        gl::EnableVertexAttribArray(attrib_location);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_pos);
        gl::VertexAttribPointer(attrib_location, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());

        let attrib_location = get_attrib_location(program, "texcoord");

        gl::EnableVertexAttribArray(attrib_location);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_tex);
        gl::VertexAttribPointer(attrib_location, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());

        vao
    }
}

fn create_shader(shader_text: &str, shader_type: u32) -> u32 {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(
            shader,
            1,
            [CString::new(shader_text).unwrap().as_ptr() as *const i8].as_ptr(),
            ptr::null(),
        );
        gl::CompileShader(shader);
        /* Check for errors in compilation */
        let mut success: i32 = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success as u8 == gl::FALSE {
            /* There was an error */
            let mut length: i32 = 0;
            let mut log = [0u8; 1024];
            gl::GetShaderInfoLog(
                shader,
                log.len() as i32,
                &mut length,
                log.as_mut_ptr() as *mut i8,
            );

            /* Transform the log from a pointer to a str */
            let log = from_utf8(CStr::from_ptr(mem::transmute(&log)).to_bytes()).unwrap();

            panic!(format!("Error while compiling shader:\n{}", log))
        } else {
            shader
        }
    }
}
