use std::{ffi::c_void, path::Path};

use gl::types::*;
use image::{ColorType, GenericImage, ImageFormat};

pub unsafe fn load_texture(image_path: &str, texture_id: GLenum, flipped: bool) -> u32 {
    let mut id = 0;
    gl::GenTextures(1, &mut id);

    gl::ActiveTexture(texture_id);
    gl::BindTexture(gl::TEXTURE_2D, id);

    let mut img = image::open(&Path::new(image_path))
        .expect("Failed to load texture");
    if flipped {
        img = img.flipv();
    }
    let data = img.raw_pixels();

    let channels = match img.color() {
        ColorType::RGBA(_) => gl::RGBA,
        ColorType::RGB(_) => gl::RGB,
        _ => panic!(format!("Invalid image color type: {:?}", img.color()))
    };

    // set texture wrapping and filtering options
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);	
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    
    // bind image data
    gl::TexImage2D(
        gl::TEXTURE_2D, 
        0, 
        gl::RGB as i32, 
        img.width() as i32,
        img.height() as i32,
        0,
        channels,
        gl::UNSIGNED_BYTE,
        &data[0] as *const u8 as *const c_void
    );
    gl::GenerateMipmap(gl::TEXTURE_2D);
    texture_id - gl::TEXTURE0
}