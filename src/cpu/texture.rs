use std::path::Path;

use image::{self, DynamicImage, GenericImageView};

#[derive(Clone)]
pub struct Texture {
    pub id: usize,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub format: TextureFormat,
}

impl Texture {
    pub fn from_path(path: &Path, id: usize, srgb: bool) -> Result<Self, std::io::Error> {
        let img = image::open(path).ok().ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Failed to load image at path: {}", path.to_string_lossy()),
        ))?;
        let (width, height) = img.dimensions();
        let format = img.color();
        let data = img.into_bytes();

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
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "Unsupported image format at path: {}",
                        path.to_string_lossy()
                    ),
                ));
            }
        };

        Ok(Self {
            id,
            width,
            height,
            data,
            format,
        })
    }

    pub fn from_dynamic_image(
        img: DynamicImage,
        id: usize,
        format: TextureFormat,
    ) -> Result<Self, std::io::Error> {
        match format {
            TextureFormat::Rgb8 => {
                let rgb = img.to_rgb8();
                let (w, h) = rgb.dimensions();
                Ok(Texture {
                    id,
                    width: w,
                    height: h,
                    data: img.into_bytes(),
                    format,
                })
            }
            TextureFormat::Rgba8 => {
                let rgba = img.to_rgba8();
                let (w, h) = rgba.dimensions();
                Ok(Texture {
                    id,
                    width: w,
                    height: h,
                    data: img.into_bytes(),
                    format,
                })
            }

            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Unsupported image format",
            )),
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
