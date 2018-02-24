extern crate gl;
extern crate glfw;
extern crate image;
extern crate cgmath;
extern crate rand;

mod util;
mod shader;
mod maze;

use std::mem;
use std::path::Path;
use std::ptr;
use std::ffi::CStr;

use image::GenericImage;
use cgmath::{Matrix4, Deg, Rad, perspective, Point3, vec3};
use cgmath::prelude::*;
use glfw::{Action, Context, Key};
use gl::types::*;

use shader::Shader;
use maze::Maze;


const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const VERTICES: [f32; 20] = [
    -0.5, -0.5, 0.0,  0.0, 0.0,
    -0.5,  0.5, 0.0,  0.0, 1.0,
     0.5,  0.5, 0.0,  1.0, 1.0,
     0.5, -0.5, 0.0,  1.0, 0.0
];
const INDICES: [u32; 6] = [
    0, 1, 3,
    1, 2, 3
];


fn main() {
    let m = Maze::new(10, 10);
    println!("{:?}", m);
    println!();
    m.print();

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    if cfg!(target_os = "macos") {
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    }

    let (mut window, events) =
        glfw.create_window(WIDTH,
                           HEIGHT,
                           "Hello this is window",
                           glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let (shader_program, ebo, texture1, texture2) = unsafe { set_up() };


    let positions = [
        vec3( 0.0,  0.0,  0.0),
        vec3( 2.0,  5.0, -15.0),
        vec3(-1.5, -2.2, -2.5),
        vec3(-3.8, -2.0, -12.3)
    ];

    let ratio = WIDTH as f32 / HEIGHT as f32;

    let mut view_pos = Point3::new(0.0, 0.0, 3.0);
    let view_dir = vec3(0.0, 0.0, -1.0);
    let view_up = vec3(0.0, 1.0, 0.0);

    let proj = perspective(Deg(45.0), ratio, 0.1, 100.0);

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        let t = glfw.get_time();

        if window.get_key(Key::W) == Action::Press {
            view_pos += 0.1 * view_dir;
        }
        if window.get_key(Key::S) == Action::Press {
            view_pos -= 0.1 * view_dir;
        }
        if window.get_key(Key::A) == Action::Press {
            view_pos -= 0.1 * view_dir.cross(view_up).normalize();
        }
        if window.get_key(Key::D) == Action::Press {
            view_pos += 0.1 * view_dir.cross(view_up).normalize();
        }

        let view = Matrix4::look_at(view_pos, view_pos + view_dir, view_up);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            shader_program.use_program();
            shader_program.set_float(c_str!("t"), t as f32);

            shader_program.set_mat4(c_str!("view"), view);
            shader_program.set_mat4(c_str!("proj"), proj);

            for v in positions.iter() {
                let model = Matrix4::from_translation(*v) *
                            Matrix4::from_angle_x(Rad(t as f32));
                shader_program.set_mat4(c_str!("model"), model);
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            }
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

unsafe fn set_up() -> (Shader, GLuint, GLuint, GLuint) {
    gl::Enable(gl::DEPTH_TEST);

    // vertex buffer obj
    let mut vbo = 0;
    gl::GenBuffers(1, &mut vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(gl::ARRAY_BUFFER,
                   mem::size_of::<[f32; 180]>() as isize,
                   &VERTICES[0] as *const f32 as *const _,
                   gl::STATIC_DRAW);


    let mut vao = 0;
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);
    // aPos = 0
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE,
                            5 * mem::size_of::<GLfloat>() as GLint,
                            ptr::null());
    gl::EnableVertexAttribArray(0);
    // aTex = 1
    gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE,
                            5 * mem::size_of::<GLfloat>() as GLint,
                            (3 * mem::size_of::<GLfloat>()) as *const GLvoid);
    gl::EnableVertexAttribArray(1);


    let mut ebo = 0;
    gl::GenBuffers(1, &mut ebo);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                   mem::size_of::<[u32; 6]>() as isize,
                   &INDICES[0] as *const u32 as *const _,
                   gl::STATIC_DRAW);


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
    shader_program.set_int(c_str!("tex1"), 0);
    shader_program.set_int(c_str!("tex2"), 1);

    (shader_program, ebo, texture1, texture2)
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
