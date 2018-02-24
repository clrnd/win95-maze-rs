use std::mem;
use std::ptr;
use std::ffi::CStr;

use gl;
use gl::types::*;
use cgmath::{Matrix4, Deg, Rad, perspective, Vector3, vec3, One};

use shader::Shader;

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

pub enum Dir {
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
    dir: Dir,
    texture: Tex,
    vbo: GLuint,
    vao: GLuint,
    ebo: GLuint
}

impl Wall {
    pub fn new(pos: Vector3<f32>, dir: Dir, texture: Tex) -> Wall {
        Wall {
            pos: pos,
            dir: dir,
            texture: texture,
            vbo: 0,
            vao: 0,
            ebo: 0,
        }
    }

    pub unsafe fn set_up(&mut self) {

        /// generate VAO, VBO, EBO
        gl::GenVertexArrays(1, &mut self.vao);
        gl::GenBuffers(1, &mut self.vbo);
        gl::GenBuffers(1, &mut self.ebo);

        /// binding VAO
        gl::BindVertexArray(self.vao);

        /// VBO data
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       mem::size_of::<[f32; 20]>() as isize,
                       &VERTICES[0] as *const f32 as *const _,
                       gl::STATIC_DRAW);

        /// EBO data
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       mem::size_of::<[u32; 6]>() as isize,
                       &INDICES[0] as *const u32 as *const _,
                       gl::STATIC_DRAW);

        /// vertex attribs
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

        /// unbind for safeness
        gl::BindVertexArray(0);
    }

    pub unsafe fn draw(&self, shader_program: &Shader) {
        let mut model = Matrix4::from_translation(self.pos);

        match self.dir {
            Dir::Vertical => {
                model = model * Matrix4::from_angle_y(Deg(90.0));
            },
            _ => {}
        };

        shader_program.set_mat4(c_str!("model"), model);

        match self.texture {
            Tex::Thing => shader_program.set_bool(c_str!("is_thing"), true),
            _ => {}
        };

        gl::BindVertexArray(self.vao);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
    }
}
