
use gl;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str::from_utf8;
use std::ffi::{CStr, CString};

static vert_shader: &'static str = include_str!("shaders/vert_shader.glsl");
static frag_shader: &'static str = include_str!("shaders/frag_shader.glsl");

pub fn clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn create_vbo() -> u32 {
    unsafe {
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let vertex_buffer = vec![ -0.5, -0.5, 0.5, -0.5, 0.0, 0.75 ];
        let vertex_data: &[f32] = vertex_buffer.as_slice();

        let data_length = (vertex_data.len() * mem::size_of::<f32>()) as gl::types::GLsizeiptr;
        let data_pointer = vertex_data.as_ptr() as *const c_void;
        gl::BufferData(
            gl::ARRAY_BUFFER,
            data_length,
            data_pointer,
            gl::STATIC_DRAW
        );
        vbo
    }
}

fn get_attrib_location(program: u32, attribute: &str) -> u32 {
    unsafe {
        gl::GetAttribLocation(program,
            CString::new(attribute).unwrap().as_ptr() as *const i8) as u32
    }
}

pub fn create_vao(program: u32, vbo: u32) -> u32 {
    unsafe {
        let mut vao = 0;
        let attrib_location = get_attrib_location(program, "position");

        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::EnableVertexAttribArray(attrib_location);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::VertexAttribPointer(attrib_location, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());

        vao
    }
}

pub fn create_shader(shader_text: &str, shader_type: u32) -> u32 {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, [CString::new(shader_text).unwrap().as_ptr() as *const i8].as_ptr(), ptr::null());
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

pub fn create_program() -> u32 {
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

pub fn draw_triangle(program: u32, vao: u32) {
    unsafe {
        gl::UseProgram(program);
        gl::BindVertexArray(vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
    }
}
