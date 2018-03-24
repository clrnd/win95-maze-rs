use std::mem;
use std::ptr;
use std::collections::HashMap;
use std::ffi::CStr;

use gl;
use gl::types::*;
use cgmath::{Matrix4, Deg, Vector3};

use texture::Texture;
use shader::Shader;

// texture coordinates are weird because somehow
// images are flipped on the x axis
const VERTICES: [f32; 20] = [
     0.5, -0.5, 0.0,  0.0, 1.0, // bottom right
    -0.5, -0.5, 0.0,  1.0, 1.0, // bottom left
    -0.5,  0.5, 0.0,  1.0, 0.0, // top left
     0.5,  0.5, 0.0,  0.0, 0.0, // top right
];
const INDICES: [u32; 6] = [
    0, 1, 3,
    1, 2, 3
];

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum TexType {
    Brick,
    Thing,
    Ceiling,
    Floor
}

#[derive(Debug)]
pub struct Wall {
    pub pos: Vector3<f32>,
    pub angle_y: f32,
    pub angle_x: f32,
    pub textype: TexType
}

#[derive(Debug)]
pub struct WallRenderer {
    textures: HashMap<TexType, Texture>,
    brick_walls: Vec<Wall>,
    thing_walls: Vec<Wall>,
    others: Vec<Wall>,
    vao: GLuint,
    vbo: GLuint
}

impl WallRenderer {

    pub unsafe fn new(textures: HashMap<TexType, Texture>) -> WallRenderer {

        let (mut vao, mut vbo, mut ebo) = (0, 0, 0);

        //* generate VAO, VBO, EBO
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        //* binding VAO
        gl::BindVertexArray(vao);

        //* VBO data
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       mem::size_of::<[f32; 20]>() as isize,
                       &VERTICES[0] as *const f32 as *const _,
                       gl::STATIC_DRAW);

        //* EBO data
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       mem::size_of::<[u32; 6]>() as isize,
                       &INDICES[0] as *const u32 as *const _,
                       gl::STATIC_DRAW);

        //* vertex attribs
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

        WallRenderer {
            textures: textures,
            brick_walls: Vec::new(),
            thing_walls: Vec::new(),
            others: Vec::new(),
            vao: vao,
            vbo: vbo
        }
    }

    pub fn add(&mut self, wall: Wall) {
        match wall.textype {
            TexType::Brick => self.brick_walls.push(wall),
            TexType::Thing => self.thing_walls.push(wall),
            _ => self.others.push(wall)
        }
    }

    pub unsafe fn draw(&self, shader_program: &Shader) {

        let draw_wall = |wall: &Wall| {
            let model = Matrix4::from_translation(wall.pos) *
                        Matrix4::from_angle_y(Deg(wall.angle_y)) *
                        Matrix4::from_angle_x(Deg(wall.angle_x));

            shader_program.set_mat4(c_str!("model"), model);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        };

        gl::BindVertexArray(self.vao);
        shader_program.set_bool(c_str!("solid"), false);

        // Set coordinates of VBO for non-tiled textures
        shader_program.set_int(c_str!("tiling"), 1);
        shader_program.set_int(
            c_str!("tex"), self.textures[&TexType::Brick].number as i32);
        self.brick_walls.iter().for_each(&draw_wall);

        shader_program.set_int(
            c_str!("tex"), self.textures[&TexType::Thing].number as i32);
        self.thing_walls.iter().for_each(&draw_wall);

        // Set coordinates of VBO for tiled textures
        shader_program.set_int(c_str!("tiling"), 4);

        for w in &self.others {
            shader_program.set_int(
                c_str!("tex"), self.textures[&w.textype].number as i32);
            draw_wall(w)
        }
    }
}
