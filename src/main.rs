extern crate gl;
extern crate glfw;
extern crate image;
extern crate cgmath;
extern crate rand;

mod util;
mod shader;
mod maze;
mod wall;
mod walker;
mod camera;

use std::path::Path;
use std::ffi::CStr;

use image::GenericImage;
use cgmath::{Matrix4, Deg, perspective, Point3, vec3};
use glfw::{Action, Context, Key};
use gl::types::*;

use wall::{Wall, WallRenderer, Kind, Tex};
use shader::Shader;
use maze::Maze;
use walker::Walker;
use camera::Camera;


const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;


fn main() {
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

    // vsync
    //glfw.set_swap_interval(glfw::SwapInterval::None);

    let maze = Maze::new(20, 20);
    maze.print();

    let (shader_program, mut wall_renderer) = unsafe {
        (set_up_shaders(), WallRenderer::new())
    };

    build_walls(&maze, &mut wall_renderer);

    let mut walker = Walker::new(&maze);

    let ratio = WIDTH as f32 / HEIGHT as f32;

    let mut camera = Camera::new(&walker);

    let proj = perspective(Deg(45.0), ratio, 0.1, 100.0);

    let mut frame_count = 0;
    let mut last_second = glfw.get_time();
    let mut last_frame = glfw.get_time();

    walker.next();
    while !window.should_close() {
        // input and stuff
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        let current_time = glfw.get_time();
        let delta_time = current_time - last_frame;
        last_frame = current_time;

        let arrived = update_camera(&mut camera, &walker, delta_time as f32);
        if arrived {
            walker.next();
        }

        let view = Matrix4::look_at(camera.pos,
                                    camera.pos + camera.dir,
                                    camera.up);

        // FPS counting
        if (current_time - last_second) > 1.0 {
            last_second = current_time;
            println!("FPS: {}", frame_count);
            frame_count = 0;
        } else {
            frame_count += 1;
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader_program.set_mat4(c_str!("view"), view);
            shader_program.set_mat4(c_str!("proj"), proj);

            wall_renderer.draw(&shader_program);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn update_camera(camera: &mut Camera, walker: &Walker, dt: f32) -> bool {
    let v_dir = walker.direction.to_vec();

    if camera.looking_at(v_dir) {
        let p_to = Point3::new(walker.j as f32 + 0.5,
                               0.0,
                               walker.i as f32 + 0.5);
        camera.move_to(p_to, dt)
    } else {
        camera.rotate_to(v_dir, dt);
        false
    }
}

fn build_walls(maze: &Maze, wall_renderer: &mut WallRenderer) {
    // top walls
    for j in 0..maze.width-1 {
        let tex = get_rand_tex();
        wall_renderer.add(
            Wall::new(vec3(j as f32 + 0.5, 0.0, 0.0),
                      Kind::Horizontal, tex))
    }

    // left walls
    for i in 0..maze.height-1 {
        let tex = get_rand_tex();
        wall_renderer.add(
            Wall::new(vec3(0.0, 0.0, i as f32 + 0.5),
                      Kind::Vertical, tex))
    }

    for i in 0..maze.height-1 {
        for j in 0..maze.width-1 {
            let tex = get_rand_tex();

            if maze.south(i, j) {
                wall_renderer.add(
                    Wall::new(vec3(j as f32 + 0.5, 0.0, i as f32 + 1.0),
                              Kind::Horizontal, tex));
            }
            if maze.east(i, j) {
                wall_renderer.add(
                    Wall::new(vec3(j as f32 + 1.0, 0.0, i as f32 + 0.5),
                              Kind::Vertical, tex));
            }
        }
    }
}

fn get_rand_tex() -> Tex {
    if rand::random::<f32>() < 0.95 {
        Tex::Brick
    } else {
        Tex::Thing
    }
}

unsafe fn set_up_shaders() -> Shader {
    gl::Enable(gl::DEPTH_TEST);

    // texture 1
    let mut texture1 = 0;
    gl::GenTextures(1, &mut texture1);
    gl::BindTexture(gl::TEXTURE_2D, texture1);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
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
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
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
    //shader_program.set_int(c_str!("tex1"), 0);
    //shader_program.set_int(c_str!("tex2"), 1);

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

//fn handle_input(window: &glfw::Window, camera: &mut Camera, speed: f32) {
//    let right = camera.dir.cross(camera.up).normalize();
//    let turn_speed = 40.0;
//
//    if window.get_key(Key::W) == Action::Press {
//        camera.pos += speed * camera.dir;
//    }
//    if window.get_key(Key::S) == Action::Press {
//        camera.pos -= speed * camera.dir;
//    }
//    if window.get_key(Key::A) == Action::Press {
//        camera.pos -= speed * right;
//    }
//    if window.get_key(Key::D) == Action::Press {
//        camera.pos += speed * right;
//    }
//    if window.get_key(Key::Up) == Action::Press {
//        camera.dir = Matrix3::from_axis_angle(right, Deg(speed * turn_speed))
//                   * camera.dir;
//    }
//    if window.get_key(Key::Right) == Action::Press {
//        camera.dir = Matrix3::from_angle_y(Deg(speed * -turn_speed))
//                   * camera.dir;
//    }
//    if window.get_key(Key::Down) == Action::Press {
//        camera.dir = Matrix3::from_axis_angle(right, Deg(speed * -turn_speed))
//                   * camera.dir;
//    }
//    if window.get_key(Key::Left) == Action::Press {
//        camera.dir = Matrix3::from_angle_y(Deg(speed * turn_speed))
//                   * camera.dir;
//    }
//}
