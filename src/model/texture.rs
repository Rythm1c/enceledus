use std::path::Path;

use glow::HasContext;
use image::GenericImageView;

use super::loader::GltfFile;

#[derive(Clone, Copy)]
pub struct Texture {
    id: glow::Texture,
    width: u32,
    height: u32,
}

impl Texture {
    // upload cpu texture data to the gpu
    pub fn from_gltf(gl: &glow::Context, texture: &gltf::Texture, file: &GltfFile) -> Self {
        let data = texture_data_from_gltf(texture, file);

        Self::upload(gl, &data)
    }

    fn upload(gl: &glow::Context, cpu: &TextureData) -> Self {
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
                glow::PixelUnpackData::Slice(Some(&cpu.pixels)),
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

fn texture_data_from_gltf(texture: &gltf::Texture, file: &GltfFile) -> TextureData {
    let src = texture.source().source();

    match src {
        gltf::image::Source::Uri { uri, .. } => {
            let parent = Path::new(&file.get_folder()[..]);
            TextureData::from_path(parent.join(uri).as_path(), false)
        }

        gltf::image::Source::View { view, mime_type } => {
            let buffer = &file.get_buffers()[view.buffer().index()];
            let start = view.offset();
            let end = start + view.length();
            let image_bytes = &buffer[start..end];

            match mime_type {
                "image/jpeg" => TextureData::from_dynamic_image(
                    image::load_from_memory_with_format(image_bytes, image::ImageFormat::Jpeg)
                        .unwrap(),
                    TextureFormat::Rgb8,
                ),
                "image/png" => TextureData::from_dynamic_image(
                    image::load_from_memory_with_format(image_bytes, image::ImageFormat::Png)
                        .unwrap(),
                    TextureFormat::Rgba8,
                ),
                _ => panic!("unsupported image type"),
            }
        }
    }
}

#[derive(Clone)]
struct TextureData {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
    format: TextureFormat,
}

impl TextureData {
    fn from_path(path: &Path, srgb: bool) -> Self {
        let img = image::open(path)
            .expect(format!("unable to open texture in {}", path.to_string_lossy()).as_str());

        let (width, height) = img.dimensions();
        let format = img.color();
        let pixels = img.into_bytes();

        let format = match format {
            image::ColorType::L8 => TextureFormat::R8,
            image::ColorType::Rgb8 => {
                if srgb {
                    TextureFormat::SrgbRgb8
                } else {
                    TextureFormat::Rgb8
                }
            }
            image::ColorType::Rgba8 => {
                if srgb {
                    TextureFormat::SrgbRgba8
                } else {
                    TextureFormat::Rgba8
                }
            }
            _ => {
                panic!(
                    "Unsupported image format at path: {}",
                    path.to_string_lossy()
                )
            }
        };

        Self {
            width,
            height,
            pixels,
            format,
        }
    }

    pub fn from_dynamic_image(img: image::DynamicImage, format: TextureFormat) -> Self {
        match format {
            TextureFormat::Rgb8 => {
                let rgb = img.to_rgb8();
                let (w, h) = rgb.dimensions();
                TextureData {
                    width: w,
                    height: h,
                    pixels: img.into_bytes(),
                    format,
                }
            }
            TextureFormat::Rgba8 => {
                let rgba = img.to_rgba8();
                let (w, h) = rgba.dimensions();
                TextureData {
                    width: w,
                    height: h,
                    pixels: img.into_bytes(),
                    format,
                }
            }

            _ => panic!("Unsupported image format"),
        }
    }
}

#[derive(Clone)]
pub enum TextureFormat {
    R8,
    Rgb8,
    Rgba8,
    SrgbRgb8,
    SrgbRgba8,
}
