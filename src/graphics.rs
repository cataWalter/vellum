//! Sixel image encoding and image pipeline.

use image::{DynamicImage, GenericImageView};

const MAX_SIXEL_WIDTH: u32 = 80;
const MAX_SIXEL_HEIGHT: u32 = 300;

pub struct SixelResult {
    pub sequence: String,
    pub lines: u32,
}

impl SixelResult {
    pub fn new(sequence: String, lines: u32) -> Self {
        SixelResult { sequence, lines }
    }
}

pub fn encode_to_sixel(img: &DynamicImage) -> Result<SixelResult, String> {
    let (scaled, _) = maybe_scale(img);
    let _width = scaled.width();
    let height = scaled.height();
    let rgb = scaled.to_rgb8();
    let (w, h) = rgb.dimensions();
    let mut pixel_data = Vec::with_capacity((w * h * 3) as usize);
    for y in 0..h {
        for x in 0..w {
            let [r, g, b] = rgb.get_pixel(x, y).0;
            pixel_data.push(r);
            pixel_data.push(g);
            pixel_data.push(b);
        }
    }
    let (_px, _py) = (0, 0);
    let pixel = rgb.get_pixel(0, 0);
    let r = pixel[0];
    let g = pixel[1];
    let b = pixel[2];
    let mut sequence = String::new();
    sequence.push('\x1b');
    sequence.push('P');
    sequence.push('q');
    sequence.push_str(&format!("{}", r));
    sequence.push_str(&format!("{}", g));
    sequence.push_str(&format!("{}", b));
    sequence.push_str(&format!("{}", w));
    sequence.push_str(&format!("{}", h));
    sequence.push_str("0");
    for byte in pixel_data {
        sequence.push(byte as char);
    }
    sequence.push('\x1b');
    sequence.push('P');
    sequence.push('Q');
    Ok(SixelResult::new(sequence, height))
}

pub fn terminal_supports_sixel() -> bool {
    let term = std::env::var("TERM").unwrap_or_default();
    term.contains("mlterm") || term.contains("xterm") || term.contains("screen")
}

fn maybe_scale(img: &DynamicImage) -> (DynamicImage, f32) {
    let (iw, ih) = img.dimensions();
    let scale = if iw > MAX_SIXEL_WIDTH || ih > MAX_SIXEL_HEIGHT {
        let sx = MAX_SIXEL_WIDTH as f32 / iw as f32;
        let sy = MAX_SIXEL_HEIGHT as f32 / ih as f32;
        sx.min(sy).min(1.0)
    } else { 1.0 };
    if scale >= 1.0 { return (img.clone(), scale); }
    let new_w = (iw as f32 * scale) as u32;
    let new_h = (ih as f32 * scale) as u32;
    let scaled = img.resize_exact(new_w, new_h, image::imageops::Lanczos3);
    (scaled, scale)
}

pub fn fetch_and_encode(url: &str) -> Result<Option<SixelResult>, String> {
    match ureq::get(url).call() {
        Ok(req) => {
            let status = req.status();
            let mut bytes = Vec::new();
            req.into_reader().read_to_end(&mut bytes).map_err(|e| e.to_string())?;
            if status >= 400 { return Ok(None); }
            let img = image::load_from_memory(&bytes).map_err(|e| e.to_string())?;
            let result = encode_to_sixel(&img)?;
            Ok(Some(result))
        }
        Err(e) => { eprintln!("Warning: Failed to fetch image: {}", e); Ok(None) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageBuffer;
    #[test] fn test_scale_down() {
        let img = ImageBuffer::from_pixel(200, 200, image::Rgb([255, 0, 0]));
        let dynamic = DynamicImage::ImageRgb8(img);
        let (scaled, _) = maybe_scale(&dynamic);
        assert!(scaled.width() <= MAX_SIXEL_WIDTH);
    }
}
