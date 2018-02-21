extern crate gl;
extern crate glfw;

//#[macro_use]
//mod util;
mod shader;

use shader::Shader;

use std::mem;
use std::ffi::CString;
use std::ptr;
use glfw::{Action, Context, Key};
use gl::types::*;


const VERTICES: [f32; 18] = [
    -0.5, -0.5, 0.0,   1.0, 0.0, 0.0,
     0.5, -0.5, 0.0,   0.0, 1.0, 0.0,
     0.0,  0.5, 0.0,   0.0, 0.0, 1.0
];


fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    if cfg!(target_os = "macos") {
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    }

    let (mut window, events) = glfw.create_window(800, 600, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let (shader_program, vao) = unsafe {
        // vertex buffer obj
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       mem::size_of::<[f32; 18]>() as isize,
                       &VERTICES[0] as *const f32 as *const _,
                       gl::STATIC_DRAW);

        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        // aPos = 0
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE,
                                6 * mem::size_of::<GLfloat>() as GLint,
                                ptr::null());
        gl::EnableVertexAttribArray(0);
        // aColor = 1
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE,
                                6 * mem::size_of::<GLfloat>() as GLint,
                                (3 * mem::size_of::<GLfloat>()) as *const GLvoid);
        gl::EnableVertexAttribArray(1);


        // vertex shader
        let shader_program = Shader::new("shaders/vertex.glsl",
                                         "shaders/fragment.glsl");

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        (shader_program, vao)
    };

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        let t = glfw.get_time();

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shader_program.use_program();
            shader_program.set_float(&CString::new("of").unwrap(), t.sin() as f32);

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
