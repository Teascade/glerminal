pub(crate) mod textbuffermesh;
pub(crate) mod backgroundmesh;

use gl;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str::from_utf8;
use std::ffi::{CStr, CString};

pub(crate) static VERT_SHADER: &'static str = include_str!("../shaders/vert_shader.glsl");
pub(crate) static FRAG_SHADER: &'static str = include_str!("../shaders/frag_shader.glsl");
pub(crate) static BG_FRAG_SHADER: &'static str = include_str!("../shaders/bg_frag_shader.glsl");
pub(crate) static DEBUG_FRAG_SHADER: &'static str =
    include_str!("../shaders/debug_frag_shader.glsl");

pub(crate) type Matrix4 = [f32; 16];

pub(crate) type Program = u32;
pub(crate) type Vao = u32;
pub(crate) type Vbo = u32;
pub(crate) type Texture = u32;

pub trait Renderable {
    fn get_vao(&self) -> Vao;
    fn get_count(&self) -> i32;
    fn get_texture(&self) -> Option<Texture>;
}

#[cfg(test)]
pub(crate) fn get_error(headless: bool) -> Option<u32> {
    if !headless {
        unsafe {
            let error = gl::GetError();
            if error == gl::NO_ERROR {
                None
            } else {
                Some(error)
            }
        }
    } else {
        None
    }
}

pub(crate) fn clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub(crate) fn update_viewport(dimensions: (u32, u32)) {
    let (width, height) = dimensions;
    unsafe {
        gl::Viewport(0, 0, width as i32, height as i32);
    }
}

pub(crate) fn set_debug(debug: bool) {
    unsafe {
        if debug {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        } else {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }
    }
}

pub(crate) fn draw(program: Program, proj_matrix: Matrix4, time: f32, renderable: &Renderable) {
    unsafe {
        gl::UseProgram(program);
        if let Some(texture) = renderable.get_texture() {
            gl::BindTexture(gl::TEXTURE_2D, texture);
        }
        gl::BindVertexArray(renderable.get_vao());

        let loc = get_uniform_location("proj_mat".to_owned(), program);
        gl::UniformMatrix4fv(loc, 1, gl::TRUE, proj_matrix.as_ptr());

        let loc = get_uniform_location("time".to_owned(), program);
        gl::Uniform1fv(loc, 1, vec![time].as_ptr());

        gl::DrawArrays(gl::TRIANGLES, 0, renderable.get_count());
    }
}

pub(crate) fn create_proj_matrix(dimensions: (f32, f32), aspect_ratio: f32) -> Matrix4 {
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

pub(crate) fn create_texture(pixels: &[u8], width: u32, height: u32) -> Texture {
    unsafe {
        let mut tex = 0;
        gl::GenTextures(1, &mut tex);
        gl::BindTexture(gl::TEXTURE_2D, tex);

        // filters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

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

pub(crate) fn upload_buffer(vbo: Vbo, vertex_buffer: Vec<f32>) {
    let data_length = (vertex_buffer.len() * mem::size_of::<f32>()) as gl::types::GLsizeiptr;
    let data_pointer = vertex_buffer.as_ptr() as *const c_void;

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, data_length, data_pointer, gl::STREAM_DRAW);
    }
}

pub(crate) fn create_vbo(vertex_buffer: Vec<f32>) -> Vbo {
    unsafe {
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let vertex_data: &[f32] = vertex_buffer.as_slice();

        let data_length = (vertex_data.len() * mem::size_of::<f32>()) as gl::types::GLsizeiptr;
        let data_pointer = vertex_data.as_ptr() as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, data_length, data_pointer, gl::STREAM_DRAW);
        vbo
    }
}

pub(crate) fn create_vao(
    program: Program,
    vbo_pos: Vbo,
    vbo_col: Vbo,
    vbo_tuple: Option<(Vbo, Vbo)>,
) -> Vao {
    unsafe {
        let mut vao = 0;

        gl::GenVertexArrays(1, &mut vao);

        gl::BindVertexArray(vao);

        let attrib_location = get_attrib_location(program, "position");
        gl::EnableVertexAttribArray(attrib_location);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_pos);
        gl::VertexAttribPointer(attrib_location, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());

        let attrib_location = get_attrib_location(program, "color");

        gl::EnableVertexAttribArray(attrib_location);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_col);
        gl::VertexAttribPointer(attrib_location, 4, gl::FLOAT, gl::FALSE, 0, ptr::null());

        if let Some((vbo_tex, vbo_shakiness)) = vbo_tuple {
            let attrib_location = get_attrib_location(program, "texcoord");

            gl::EnableVertexAttribArray(attrib_location);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_tex);
            gl::VertexAttribPointer(attrib_location, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());

            let attrib_location = get_attrib_location(program, "shakiness");

            gl::EnableVertexAttribArray(attrib_location);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_shakiness);
            gl::VertexAttribPointer(attrib_location, 1, gl::FLOAT, gl::FALSE, 0, ptr::null());
        }

        vao
    }
}

pub(crate) fn create_program(vert_shader: &str, frag_shader: &str) -> Program {
    unsafe {
        let vert = create_shader(vert_shader, gl::VERTEX_SHADER);
        let frag = create_shader(frag_shader, gl::FRAGMENT_SHADER);

        let program = gl::CreateProgram();

        gl::AttachShader(program, vert);
        gl::AttachShader(program, frag);

        gl::LinkProgram(program);

        program
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

unsafe fn get_attrib_location(program: Program, attribute: &str) -> u32 {
    gl::GetAttribLocation(
        program,
        CString::new(attribute).unwrap().as_ptr() as *const i8,
    ) as u32
}

unsafe fn get_uniform_location(uniform: String, program: Program) -> i32 {
    gl::GetUniformLocation(
        program,
        CString::new(uniform.to_string()).unwrap().as_ptr() as *const i8,
    ) as i32
}
