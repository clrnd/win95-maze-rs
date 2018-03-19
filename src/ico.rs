use std::mem;
use std::ptr;
use std::ffi::CStr;

use gl;
use gl::types::*;
use cgmath::{Matrix4, Deg, Vector3};

use shader::Shader;

const VERTICES: [f32; 36] = [
     0.0     ,  0.0    , -1.0     ,
     0.7236  , -0.52572, -0.447215,
    -0.276385, -0.85064, -0.447215,
    -0.894425,  0.0    , -0.447215,
    -0.276385,  0.85064, -0.447215,
     0.7236  ,  0.52572, -0.447215,
     0.276385, -0.85064,  0.447215,
    -0.7236  , -0.52572,  0.447215,
    -0.7236  ,  0.52572,  0.447215,
     0.276385,  0.85064,  0.447215,
     0.894425,  0.     ,  0.447215,
     0.0     ,  0.0    ,  1.0
];
const INDICES: [u32; 60] = [
     0,  1,  2,
     1,  0,  5,
     0,  2,  3,
     0,  3,  4,
     0,  4,  5,
     1,  5, 10,
     2,  1,  6,
     3,  2,  7,
     4,  3,  8,
     5,  4,  9,
     1, 10,  6,
     2,  6,  7,
     3,  7,  8,
     4,  8,  9,
     5,  9, 10,
     6, 10, 11,
     7,  6, 11,
     8,  7, 11,
     9,  8, 11,
    10,  9, 11,
];

#[derive(Debug)]
pub struct Ico {
    pub pos: Vector3<f32>
}

#[derive(Debug)]
pub struct IcoRenderer {
    icos: Vec<Ico>,
    vao: GLuint,
}

impl IcoRenderer {

    pub unsafe fn new() -> IcoRenderer {
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
                       mem::size_of::<[f32; 36]>() as isize,
                       &VERTICES[0] as *const f32 as *const _,
                       gl::STATIC_DRAW);

        //* EBO data
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       mem::size_of::<[u32; 60]>() as isize,
                       &INDICES[0] as *const u32 as *const _,
                       gl::STATIC_DRAW);

        //* vertex attribs
        // aPos = 0
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE,
                                3 * mem::size_of::<GLfloat>() as GLint,
                                ptr::null());
        gl::EnableVertexAttribArray(0);

        //* unbind for safeness
        gl::BindVertexArray(0);

        IcoRenderer {
            icos: Vec::new(),
            vao: vao
        }
    }

    pub fn add(&mut self, ico: Ico) {
        self.icos.push(ico)
    }

    pub unsafe fn draw(&self, shader_program: &Shader) {

        gl::BindVertexArray(self.vao);
        shader_program.set_bool(c_str!("solid"), true);

        for w in &self.icos {
            let model = Matrix4::from_translation(w.pos);
            shader_program.set_mat4(c_str!("model"), model);
            gl::DrawElements(gl::TRIANGLES, 60, gl::UNSIGNED_INT, ptr::null());
        }

        gl::BindVertexArray(0);
    }
}
