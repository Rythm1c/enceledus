use image::{self, GenericImageView};

#[derive(Clone)]
pub struct CPUTexture {
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub format: TextureFormat,
}

impl CPUTexture {
    pub fn new(path: &str, srgb: bool) -> Result<Self, std::io::Error> {
        let img = image::open(path).ok().ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Failed to load image at path: {}", path),
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
                    format!("Unsupported image format at path: {}", path),
                ));
            }
        };

        Ok(Self {
            path: String::from(path),
            width,
            height,
            data,
            format,
        })
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
