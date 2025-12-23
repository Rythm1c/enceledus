use glow::HasContext;

use crate::src::cpu::texture::Texture;
use crate::src::cpu::texture::TextureFormat;

#[derive(Clone, Copy)]
pub struct GpuTexture {
    id: glow::Texture,
    width: u32,
    height: u32,
}

impl GpuTexture {
    // upload cpu texture data to the gpu
    pub fn from_cpu(gl: &glow::Context, cpu: &Texture) -> Self {
        unsafe {
            let tex = gl.create_texture().expect("Failed to create texture");
            gl.bind_texture(glow::TEXTURE_2D, Some(tex));

            let (internal, format) = match cpu.format {
                TextureFormat::Rgb8 => (glow::RGB8, glow::RGB),
                TextureFormat::Rgba8 => (glow::RGBA8, glow::RGBA),
                TextureFormat::SrgbRgb8 => (glow::SRGB8, glow::RGB),
                TextureFormat::SrgbRgba8 => (glow::SRGB8_ALPHA8, glow::RGBA),
                TextureFormat::R8 => (glow::R8, glow::RED),
            };

            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                internal as i32,
                cpu.width as i32,
                cpu.height as i32,
                0,
                format,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(Some(&cpu.data)),
            );

            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );

            gl.generate_mipmap(glow::TEXTURE_2D);

            Self {
                id: tex,
                width: cpu.width,
                height: cpu.height,
            }
        }
    }

    pub fn bind_to_uint(&self, gl: &glow::Context, unit: u32) {
        unsafe {
            gl.active_texture(glow::TEXTURE0 + unit);
            gl.bind_texture(glow::TEXTURE_2D, Some(self.id));
        }
    }

    pub fn delete(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_texture(self.id);
        }
    }
}
