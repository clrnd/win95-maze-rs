extern crate gl;
extern crate glfw;
extern crate image;

mod util;
mod shader;

use shader::Shader;
use util::*;

use image::GenericImage;

use std::mem;
use std::path::Path;
use std::ptr;
use glfw::{Action, Context, Key};
use gl::types::*;


const VERTICES: [f32; 24] = [
    -0.5, -0.5, 0.0,  1.0, 0.0, 0.0,  0.0, 0.0,
     0.5, -0.5, 0.0,  0.0, 1.0, 0.0,  1.0, 0.0,
     0.0,  0.5, 0.0,  0.0, 0.0, 1.0,  0.5, 1.0
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

    let (shader_program, vao, texture1, texture2) = unsafe { set_up() };

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        //let t = glfw.get_time();

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            shader_program.use_program();

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

unsafe fn set_up() -> (Shader, GLuint, GLuint, GLuint) {
    // vertex buffer obj
    let mut vbo = 0;
    gl::GenBuffers(1, &mut vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(gl::ARRAY_BUFFER,
                   mem::size_of::<[f32; 24]>() as isize,
                   &VERTICES[0] as *const f32 as *const _,
                   gl::STATIC_DRAW);


    let mut vao = 0;
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);
    // aPos = 0
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE,
                            8 * mem::size_of::<GLfloat>() as GLint,
                            ptr::null());
    gl::EnableVertexAttribArray(0);
    // aColor = 1
    gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE,
                            8 * mem::size_of::<GLfloat>() as GLint,
                            (3 * mem::size_of::<GLfloat>()) as *const GLvoid);
    gl::EnableVertexAttribArray(1);
    // aTex = 2
    gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE,
                            8 * mem::size_of::<GLfloat>() as GLint,
                            (6 * mem::size_of::<GLfloat>()) as *const GLvoid);
    gl::EnableVertexAttribArray(2);


    // texture 1
    let mut texture1 = 0;
    gl::GenTextures(1, &mut texture1);
    gl::BindTexture(gl::TEXTURE_2D, texture1);
    let img = image::open(&Path::new("resources/1.bmp"))
        .expect("Failed to load texture1");
    let data = img.raw_pixels();
    gl::TexImage2D(gl::TEXTURE_2D,
                   0,
                   gl::RGB as i32,
                   img.width() as i32,
                   img.height() as i32,
                   0,
                   gl::RGB,
                   gl::UNSIGNED_BYTE,
                   &data[0] as *const u8 as *const GLvoid);
    gl::GenerateMipmap(gl::TEXTURE_2D);

    // texture 2
    let mut texture2 = 0;
    gl::GenTextures(1, &mut texture2);
    gl::BindTexture(gl::TEXTURE_2D, texture2);
    let img = image::open(&Path::new("resources/2.bmp"))
        .expect("Failed to load texture2");
    let data = img.raw_pixels();
    gl::TexImage2D(gl::TEXTURE_2D,
                   0,
                   gl::RGB as i32,
                   img.width() as i32,
                   img.height() as i32,
                   0,
                   gl::RGB,
                   gl::UNSIGNED_BYTE,
                   &data[0] as *const u8 as *const GLvoid);
    gl::GenerateMipmap(gl::TEXTURE_2D);


    // vertex shader
    let shader_program = Shader::new("shaders/vertex.glsl",
                                     "shaders/fragment.glsl");

    shader_program.use_program();
    shader_program.set_int(&c_str("tex1"), 0);
    shader_program.set_int(&c_str("tex2"), 1);

    (shader_program, vao, texture1, texture2)
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
