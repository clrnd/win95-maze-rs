use std::mem;
use std::ptr;
use std::collections::HashMap;
use std::ffi::CStr;

use gl;
use gl::types::*;
use cgmath::{Matrix4, Deg, Vector3};

use texture::{Texture, TexType};
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

#[derive(Debug)]
pub struct Wall {
    pub pos: Vector3<f32>,
    pub angle_y: f32,
    pub angle_x: f32,
    pub textype: TexType
}

#[derive(Debug)]
pub struct WallRenderer {
    last_textype: TexType,
    vao: GLuint
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
                                5 * mem::size_of::<f32>() as GLint,
                                ptr::null());
        gl::EnableVertexAttribArray(0);
        // aTex = 1
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE,
                                5 * mem::size_of::<GLfloat>() as GLint,
                                (3 * mem::size_of::<GLfloat>()) as *const GLvoid);
        gl::EnableVertexAttribArray(1);

        WallRenderer {
            last_textype: TexType::Other,
            vao: vao
        }
    }

    pub unsafe fn set_up(&self, shader_program: &Shader) {
        gl::BindVertexArray(self.vao);

        shader_program.set_bool(c_str!("rat"), false);
        shader_program.set_bool(c_str!("shaded"), false);
    }

    pub unsafe fn draw(&mut self,
                       shader_program: &Shader,
                       textures: &HashMap<TexType, Texture>,
                       wall: &Wall) {
        let model = Matrix4::from_translation(wall.pos) *
                    Matrix4::from_angle_y(Deg(wall.angle_y)) *
                    Matrix4::from_angle_x(Deg(wall.angle_x));

        shader_program.set_mat4(c_str!("model"), model);

        // only change uniforms if texture changed
        if self.last_textype != wall.textype {
            let tex = textures[&wall.textype].number as i32;
            shader_program.set_int(c_str!("tex"), tex);
            shader_program.set_int(c_str!("tiling"), wall.textype.tiling());

            self.last_textype = wall.textype;
        }

        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
    }
}
