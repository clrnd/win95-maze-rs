use std::mem;
use std::ptr;
use std::ffi::CStr;
use std::collections::HashMap;

use gl;
use gl::types::*;
use cgmath::{Matrix4, Vector3, EuclideanSpace, InnerSpace, MetricSpace};

use texture::{Texture, TexType};
use shader::Shader;
use walker::Walker;

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
const MOVE_SPEED: f32 = 3.0;

pub struct Rat<'a> {
    pub pos: Vector3<f32>,
    pub walker: Walker<'a>
}

#[derive(Debug)]
pub struct RatRenderer {
    vao: GLuint
}

impl<'a> Rat<'a> {
    pub fn update(&mut self, dt: f32) {
        let p_to = self.walker.to_point().to_vec();
        let old_dir = (p_to - self.pos).normalize();

        self.pos += MOVE_SPEED * dt * old_dir;

        // if new_dir is opposite direction from old_dir
        // then we went through, just assign it
        // and update the walker
        let new_dir = (p_to - self.pos).normalize();
        if old_dir.distance(new_dir) >= 0.5 {
            self.pos = p_to;
            self.walker.next();
        }
    }
}

impl RatRenderer {

    pub unsafe fn new() -> RatRenderer {

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
                       VERTICES.as_ptr() as *const _,
                       gl::STATIC_DRAW);

        //* EBO data
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       mem::size_of::<[u32; 6]>() as isize,
                       INDICES.as_ptr() as *const _,
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

        RatRenderer {
            vao: vao
        }
    }

    pub unsafe fn set_up(&self,
                         shader_program: &Shader,
                         textures: &HashMap<TexType, Texture>) {
        gl::BindVertexArray(self.vao);

        shader_program.set_bool(c_str!("rat"), true);
        shader_program.set_bool(c_str!("shaded"), false);

        let rat_tex = textures[&TexType::Rat].number as i32;
        shader_program.set_int(c_str!("tex"), rat_tex);
        shader_program.set_int(c_str!("tiling"), TexType::Rat.tiling());
    }

    pub unsafe fn draw(&self, shader_program: &Shader, rat: &Rat) {
        let model = Matrix4::from_translation(rat.pos);

        shader_program.set_mat4(c_str!("model"), model);

        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
    }
}
