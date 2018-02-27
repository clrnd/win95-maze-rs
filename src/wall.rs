use std::mem;
use std::ptr;
use std::ffi::CStr;

use gl;
use gl::types::*;
use cgmath::{Matrix4, Deg, Rad, perspective, Vector3, vec3, One};

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

pub enum Kind {
    Vertical,
    Horizontal
}

#[derive(Copy, Clone)]
pub enum Tex {
    Brick,
    Thing
}

pub struct Wall {
    pos: Vector3<f32>,
    kind: Kind,
    texture: Tex
}

pub struct WallRenderer {
    brick_walls: Vec<Wall>,
    thing_walls: Vec<Wall>,
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint
}

impl Wall {
    pub fn new(pos: Vector3<f32>, kind: Kind, texture: Tex) -> Wall {
        Wall {
            pos: pos,
            kind: kind,
            texture: texture
        }
    }
}

impl WallRenderer {

    pub unsafe fn new() -> WallRenderer {

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

        //* unbind for safeness
        gl::BindVertexArray(0);

        WallRenderer {
            brick_walls: Vec::new(),
            thing_walls: Vec::new(),
            vao: vao,
            vbo: vbo,
            ebo: ebo
        }
    }

    pub fn add(&mut self, wall: Wall) {
        match wall.texture {
            Tex::Brick => self.brick_walls.push(wall),
            Tex::Thing => self.thing_walls.push(wall),
        }
    }

    pub unsafe fn draw(&self, shader_program: &Shader) {

        let draw_wall = |wall: &Wall| {
            let model = match wall.kind {
                Kind::Vertical =>  Matrix4::from_translation(wall.pos)
                                * Matrix4::from_angle_y(Deg(90.0)),
                Kind::Horizontal => Matrix4::from_translation(wall.pos)
            };

            shader_program.set_mat4(c_str!("model"), model);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        };

        gl::BindVertexArray(self.vao);

        shader_program.set_int(c_str!("tex"), 0);
        for w in &self.brick_walls { draw_wall(w) };

        shader_program.set_int(c_str!("tex"), 1);
        for w in &self.thing_walls { draw_wall(w) };

        gl::BindVertexArray(0);
    }
}
