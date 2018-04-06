extern crate gl;
extern crate glfw;
extern crate image;
extern crate cgmath;
extern crate rand;

mod util;
mod ico;
mod rat;
mod shader;
mod maze;
mod wall;
mod walker;
mod camera;
mod texture;

use std::env;
use std::cmp;
use std::ffi::CStr;
use std::collections::HashMap;

use cgmath::{Matrix3, Matrix4, Deg, perspective, vec3, InnerSpace};
use glfw::{Action, Context, Key};

use wall::{Wall, WallRenderer};
use ico::{Ico, IcoRenderer};
use rat::{Rat, RatRenderer};
use shader::Shader;
use maze::Maze;
use walker::Walker;
use camera::Camera;
use texture::{Texture, TexType};


const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;


#[derive(Debug)]
enum State {
    Walking,
    Turning,
    Rolling
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    if cfg!(target_os = "macos") {
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    }

    let (mut window, events) = glfw.with_primary_monitor(
        |glfw: &mut _, m: Option<&glfw::Monitor>| {
            let fullscreen = env::args().any(|a| a == "--fullscreen");
            let (mode, w, h) = if fullscreen {
                let vid = m.unwrap().get_video_mode().unwrap();
                (glfw::WindowMode::FullScreen(m.unwrap()),
                 vid.width,
                 vid.height)
            } else {
                (glfw::WindowMode::Windowed, WIDTH, HEIGHT)
            };

            glfw.create_window(w, h, "Win95 Maze", mode)
                .expect("Failed to create GLFW window.")
        });

    let (width, height) = window.get_size();
    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // vsync off
    //glfw.set_swap_interval(glfw::SwapInterval::None);

    let maze = Maze::new(20, 20);
    maze.print();

    let mut state = State::Walking;

    let ratio = width as f32 / height as f32;
    let proj = perspective(Deg(60.0), ratio, 0.1, 100.0);

    let (shader_program, textures) = unsafe {
        (set_up_shaders(proj), set_up_textures())
    };

    let mut wall_renderer = unsafe { WallRenderer::new() };
    let walls = gen_walls(&maze);

    let ico_renderer = unsafe { IcoRenderer::new() };
    let mut icos = gen_icos(&maze);

    let rat_renderer = unsafe { RatRenderer::new() };
    let mut rats = gen_rats(&maze);

    let mut walker = Walker::new(&maze, 0, 0);
    let mut camera = Camera::new(walker.i, walker.j,
                                 walker.direction.to_vec());

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
        let delta_time = (current_time - last_frame) as f32;
        last_frame = current_time;

        // camera movement
        let completed = match state {
            State::Walking => {
                camera.move_to(walker.to_point(), delta_time)
            }
            State::Turning => {
                let v_dir = walker.direction.to_vec();
                camera.rotate_to(v_dir, delta_time)
            }
            State::Rolling => {
                let y = if camera.upside_down { 1.0 } else { -1.0 };
                camera.roll_to(vec3(0.0, y, 0.0), delta_time)
            }
        };

        // next state
        if completed {
            state = match state {
                State::Walking => {
                    walker.next();
                    let v_dir = walker.direction.to_vec();
                    if camera.looking_at(v_dir) {
                        if icos.contains_key(&walker.pos()) {
                            State::Rolling
                        } else {
                            State::Walking
                        }
                    } else {
                        State::Turning
                    }
                }
                State::Turning => {
                    if icos.contains_key(&walker.pos()) {
                        State::Rolling
                    } else {
                        State::Walking
                    }
                }
                State::Rolling => {
                    camera.upside_down = !camera.upside_down;
                    icos.remove(&walker.pos());
                    State::Walking
                }
            };
        };

        // update rats
        for rat in &mut rats {
            rat.update(delta_time);
        }

        // manual movement
        //handle_input(&window, &mut camera, delta_time * 3.0);

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

        // rendering
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // set the camera matrix
            shader_program.set_mat4(c_str!("view"), view);

            // walls have non alpha textures, and are not shaded
            shader_program.set_bool(c_str!("rat"), false);
            shader_program.set_bool(c_str!("alpha"), false);
            shader_program.set_bool(c_str!("shaded"), false);
            for wall in &walls {
                wall_renderer.draw(&shader_program, &textures, wall);
            }

            // rats have a single texture with alpha
            shader_program.set_bool(c_str!("rat"), true);
            shader_program.set_bool(c_str!("alpha"), true);
            let rat_tex = textures[&TexType::Rat].number as i32;
            shader_program.set_int(c_str!("tex"), rat_tex);
            shader_program.set_int(c_str!("tiling"), TexType::Rat.tiling());
            for rat in &rats {
                rat_renderer.draw(&shader_program, rat);
            }

            // finally, icos are shaded
            shader_program.set_bool(c_str!("rat"), false);
            shader_program.set_bool(c_str!("shaded"), true);
            for (_, ico) in &icos {
                ico_renderer.draw(&shader_program, ico, current_time as f32);
            }
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn gen_walls(maze: &Maze)  -> Vec<Wall> {

    let mut walls = Vec::new();

    // north walls
    for j in 0..maze.width {
        let tex = get_rand_tex();
        walls.push(
            Wall {
                pos: vec3(j as f32 + 0.5, 0.0, 0.0),
                angle_y: 0.0,
                angle_x: 0.0,
                textype: tex
            })
    }

    // west walls
    for i in 0..maze.height {
        let tex = get_rand_tex();
        walls.push(
            Wall {
                pos: vec3(0.0, 0.0, i as f32 + 0.5),
                angle_y: 90.0,
                angle_x: 0.0,
                textype: tex
            })
    }

    // inner walls but only east or south
    for i in 0..maze.height {
        for j in 0..maze.width {

            // south wall
            if maze.south(i, j) {
                let tex = get_rand_tex();
                walls.push(
                    Wall {
                        pos: vec3(j as f32 + 0.5, 0.0, i as f32 + 1.0),
                        angle_y: 0.0,
                        angle_x: 0.0,
                        textype: tex
                    })
            }

            // east wall
            if maze.east(i, j) {
                let tex = get_rand_tex();
                walls.push(
                    Wall {
                        pos: vec3(j as f32 + 1.0, 0.0, i as f32 + 0.5),
                        angle_y: 90.0,
                        angle_x: 0.0,
                        textype: tex
                    })
            }

            // ceiling wall
            walls.push(
                Wall {
                    pos: vec3(j as f32 + 0.5, 0.5, i as f32 + 0.5),
                    angle_y: 0.0,
                    angle_x: 90.0,
                    textype: TexType::Ceiling
                });

            // floor wall
            walls.push(
                Wall {
                    pos: vec3(j as f32 + 0.5, -0.5, i as f32 + 0.5),
                    angle_y: 0.0,
                    angle_x: 90.0,
                    textype: TexType::Floor
                });
        }
    }

    // sort walls by textype to avoid changing uniforms so much
    walls.sort_unstable_by_key(|w| w.textype);
    walls
}

fn gen_icos(maze: &Maze) -> HashMap<(usize, usize), Ico> {
    // let's say there is 6% of tiles with an icosahedron
    let total = maze.width * maze.height;
    let count = cmp::max(6 * total / 100, 2);
    let indices = rand::seq::sample_indices(
        &mut rand::thread_rng(), total, count);
    let rnd_f = || rand::random::<f32>() * 2.0 - 1.0;

    let mut icos = HashMap::new();

    for e in indices {
        let i = e / maze.width;
        let j = e % maze.width;

        icos.insert(
            (i, j),
            Ico {
                pos: vec3(j as f32 + 0.5, 0.0, i as f32 + 0.5),
                axis: vec3(rnd_f(), rnd_f(), rnd_f()).normalize(),
                active: true
            });
    }

    icos
}

fn gen_rats(maze: &Maze) -> Vec<Rat> {
    // let's say there is 2% of tiles with a rat initially
    let total = maze.width * maze.height;
    let count = cmp::max(2 * total / 100, 2);
    let indices = rand::seq::sample_indices(
        &mut rand::thread_rng(), total, count);

    let mut vec = Vec::new();

    for e in indices {
        let i = e / maze.width;
        let j = e % maze.width;
        println!("{}, {}", i, j);

        let mut walker = Walker::new(&maze, i, j);
        walker.next();
        vec.push(Rat {
            pos: vec3(j as f32 + 0.5, 0.0, i as f32 + 0.5),
            walker: walker
        });
    }
    vec
}

fn get_rand_tex() -> TexType {
    if rand::random::<f32>() < 0.9 {
        TexType::Brick
    } else {
        TexType::Thing
    }
}

unsafe fn set_up_textures() -> HashMap<TexType, Texture> {
    let mut textures = HashMap::new();
    textures.insert(TexType::Brick, Texture::new("resources/brick.bmp", 0));
    textures.insert(TexType::Thing, Texture::new("resources/thing.bmp", 1));
    textures.insert(TexType::Ceiling, Texture::new("resources/ceiling.bmp", 2));
    textures.insert(TexType::Floor, Texture::new("resources/floor.bmp", 3));
    textures.insert(TexType::Rat, Texture::new("resources/rat.bmp", 4));

    for (_, texture) in &textures {
        texture.bind();
    }

    textures
}

unsafe fn set_up_shaders(proj: Matrix4<f32>) -> Shader {
    gl::Enable(gl::DEPTH_TEST);

    // wireframes?
    //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

    let shader_program = Shader::new("shaders/vertex.glsl",
                                     "shaders/fragment.glsl");

    shader_program.use_program();
    shader_program.set_vec3(c_str!("color"), vec3(0.8, 0.1, 0.5));
    shader_program.set_mat4(c_str!("proj"), proj);

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

#[allow(dead_code)]
fn handle_input(window: &glfw::Window, camera: &mut Camera, speed: f32) {
    let right = camera.dir.cross(camera.up).normalize();
    let turn_speed = 60.0;

    if window.get_key(Key::W) == Action::Press {
        camera.pos += speed * camera.dir;
    }
    if window.get_key(Key::S) == Action::Press {
        camera.pos -= speed * camera.dir;
    }
    if window.get_key(Key::A) == Action::Press {
        camera.pos -= speed * right;
    }
    if window.get_key(Key::D) == Action::Press {
        camera.pos += speed * right;
    }
    if window.get_key(Key::Up) == Action::Press {
        camera.dir = Matrix3::from_axis_angle(right, Deg(speed * turn_speed))
                   * camera.dir;
    }
    if window.get_key(Key::Right) == Action::Press {
        camera.dir = Matrix3::from_angle_y(Deg(speed * -turn_speed))
                   * camera.dir;
    }
    if window.get_key(Key::Down) == Action::Press {
        camera.dir = Matrix3::from_axis_angle(right, Deg(speed * -turn_speed))
                   * camera.dir;
    }
    if window.get_key(Key::Left) == Action::Press {
        camera.dir = Matrix3::from_angle_y(Deg(speed * turn_speed))
                   * camera.dir;
    }
}
