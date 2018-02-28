use std::path::Path;

use gl;
use gl::types::*;
use image;
use image::GenericImage;

#[derive(Debug)]
pub struct Texture {
    pub id: GLuint,
    pub number: u32
}

impl Texture {
    pub unsafe fn new(name: &str, number: u32) -> Texture {
        let mut id = 0;
        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D, id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        let img = image::open(&Path::new(name))
            .expect("Failed to load texture.");
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

        Texture {
            id: id,
            number: number
        }
    }

    pub unsafe fn bind(&self) {
        gl::ActiveTexture(gl::TEXTURE0 + self.number);
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }
}
