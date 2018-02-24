extern crate gl;
extern crate glfw;
extern crate image;
extern crate cgmath;
extern crate rand;

mod util;
mod shader;
mod maze;
mod wall;

use std::path::Path;
use std::ffi::CStr;

use image::GenericImage;
use cgmath::{Matrix4, Matrix3, Deg, perspective, Point3, vec3, Vector3};
use cgmath::prelude::*;
use glfw::{Action, Context, Key};
use gl::types::*;

use wall::*;
use shader::Shader;
use maze::Maze;


const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct Camera {
    pos: Point3<f32>,
    dir: Vector3<f32>,
    up: Vector3<f32>
}


fn main() {
    let maze = Maze::new(10, 10);
    println!("{:?}", maze);
    println!();
    maze.print();

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

    let mut walls = Vec::with_capacity(maze.width * maze.height);

    // top walls
    for j in 0..maze.width-1 {
        walls.push(
            Wall::new(vec3(j as f32 + 0.5, 0.0, 0.0),
                      Dir::Horizontal, Tex::Brick))
    }

    // left walls
    for i in 0..maze.height-1 {
        walls.push(
            Wall::new(vec3(0.0, 0.0, i as f32 + 0.5),
                      Dir::Vertical, Tex::Brick))
    }

    for i in 0..maze.height-1 {
        for j in 0..maze.width-1 {
            let tex = if rand::random() {
                Tex::Brick
            } else {
                Tex::Thing
            };

            if maze.grid[i][j] & maze::S == 0 {
                walls.push(
                    Wall::new(vec3(j as f32 + 0.5, 0.0, i as f32 + 1.0),
                              Dir::Horizontal, tex));
            }
            if maze.grid[i][j] & maze::E == 0 {
                walls.push(
                    Wall::new(vec3(j as f32 + 1.0, 0.0, i as f32 + 0.5),
                              Dir::Vertical, tex));
            }
        }
    }

    let shader_program = unsafe {
        for w in &mut walls {
            w.set_up();
        }
        set_up()
    };

    let ratio = WIDTH as f32 / HEIGHT as f32;

    let mut camera = Camera {
        pos: Point3::new(0.0, 0.0, 3.0),
        dir: vec3(0.0, 0.0, -1.0),
        up: vec3(0.0, 1.0, 0.0)
    };

    let proj = perspective(Deg(45.0), ratio, 0.1, 100.0);

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        let t = glfw.get_time();

        handle_input(&window, &mut camera);

        let view = Matrix4::look_at(camera.pos, camera.pos + camera.dir, camera.up);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader_program.set_mat4(c_str!("view"), view);
            shader_program.set_mat4(c_str!("proj"), proj);

            for w in walls.iter() {
                w.draw(&shader_program);
            }
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

unsafe fn set_up() -> Shader {
    gl::Enable(gl::DEPTH_TEST);

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
    let img = image::open(&Path::new("resources/4.bmp"))
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


    // shaders
    let shader_program = Shader::new("shaders/vertex.glsl",
                                     "shaders/fragment.glsl");

    shader_program.use_program();
    shader_program.set_int(c_str!("tex1"), 0);
    shader_program.set_int(c_str!("tex2"), 1);

    gl::ActiveTexture(gl::TEXTURE0);
    gl::BindTexture(gl::TEXTURE_2D, texture1);
    gl::ActiveTexture(gl::TEXTURE1);
    gl::BindTexture(gl::TEXTURE_2D, texture2);

    shader_program
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}

fn handle_input(window: &glfw::Window, camera: &mut Camera) {
    let right = camera.dir.cross(camera.up).normalize();
    if window.get_key(Key::W) == Action::Press {
        camera.pos += 0.1 * camera.dir;
    }
    if window.get_key(Key::S) == Action::Press {
        camera.pos -= 0.1 * camera.dir;
    }
    if window.get_key(Key::A) == Action::Press {
        camera.pos -= 0.1 * right;
    }
    if window.get_key(Key::D) == Action::Press {
        camera.pos += 0.1 * right;
    }
    if window.get_key(Key::Up) == Action::Press {
        camera.dir = Matrix3::from_axis_angle(right, Deg(2.0)) * camera.dir;
    }
    if window.get_key(Key::Right) == Action::Press {
        camera.dir = Matrix3::from_angle_y(Deg(-2.0)) * camera.dir;
    }
    if window.get_key(Key::Down) == Action::Press {
        camera.dir = Matrix3::from_axis_angle(right, Deg(-2.0)) * camera.dir;
    }
    if window.get_key(Key::Left) == Action::Press {
        camera.dir = Matrix3::from_angle_y(Deg(2.0)) * camera.dir;
    }
}
