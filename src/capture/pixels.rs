use crate::capture::errors::ConversionError;
use image;
use std::time::SystemTime;

#[derive(Debug)]
pub struct RawCapture {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
    pub scale: f64,
}

#[derive(Debug)]
pub struct Capture {
    pub image: image::RgbaImage,
    pub scale: f64,
    pub timestamp: SystemTime,
}

impl Default for Capture {
    fn default() -> Self {
        Self {
            image: image::RgbaImage::new(0, 0),
            scale: 1.0,
            timestamp: SystemTime::UNIX_EPOCH,
        }
    }
}

impl Capture {
    pub fn crop(&self, x: f64, y: f64, width: f64, height: f64) -> Self {
        let scale = self.scale;
        let px = (x * scale).round().max(0.0) as u32;
        let py = (y * scale).round().max(0.0) as u32;
        let pw = (width * scale).round().max(0.0) as u32;
        let ph = (height * scale).round().max(0.0) as u32;

        Self {
            image: image::imageops::crop_imm(&self.image, px, py, pw, ph).to_image(),
            scale,
            timestamp: self.timestamp,
        }
    }
}

impl TryFrom<RawCapture> for Capture {
    type Error = ConversionError;

    fn try_from(raw: RawCapture) -> Result<Self, Self::Error> {
        let format = raw.format;
        let width = raw.width;
        let height = raw.height;
        let scale = raw.scale;

        let buf = match format {
            16 | 17 => format_16_17(pack(raw)),
            6 => format_6(pack(raw)),
            format => return Err(Self::Error::UnsupportedFormat(format)),
        };

        let image = image::RgbaImage::from_raw(width, height, buf)
            .ok_or(Self::Error::BadBuffer { width, height })?;

        Ok(Self {
            image,
            scale,
            timestamp: SystemTime::now(),
        })
    }
}

fn format_16_17(mut buf: Vec<u8>) -> Vec<u8> {
    for px in buf.as_chunks_mut::<4>().0 {
        px[3] = 255;
    }
    buf
}

fn format_6(mut buf: Vec<u8>) -> Vec<u8> {
    for px in buf.as_chunks_mut::<4>().0 {
        px.swap(0, 2);
        let a = px[3];
        if a != 0 && a != 255 {
            px[0] = ((px[0] as u16 * 255) / a as u16) as u8;
            px[1] = ((px[1] as u16 * 255) / a as u16) as u8;
            px[2] = ((px[2] as u16 * 255) / a as u16) as u8;
        }
    }
    buf
}

fn pack(raw: RawCapture) -> Vec<u8> {
    let row = raw.width as usize * 4;
    if raw.stride as usize == row {
        return raw.data;
    }
    let mut buf = Vec::with_capacity(row * raw.height as usize);
    for line in raw.data.chunks(raw.stride as usize) {
        buf.extend_from_slice(&line[..row]);
    }
    buf
}
