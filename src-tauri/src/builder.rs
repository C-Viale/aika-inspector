use image::{ImageBuffer, Rgba};

use crate::structs::RGBA;

pub fn build_png(pixel_buffer: &[RGBA], width: u32, height: u32) -> Result<Vec<u8>, &'static str> {
  if pixel_buffer.len() != (width * height) as usize {
    return Err("Pixel count does not match image dimensions");
  }

  let raw_pixels: Vec<u8> = pixel_buffer
    .iter()
    .flat_map(|p| [p.r as u8, p.g as u8, p.b as u8, p.a as u8])
    .collect();

  let image = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, raw_pixels).ok_or("Failed to create image buffer")?;

  let mut png_buffer = Vec::new();

  image::DynamicImage::ImageRgba8(image)
    .write_to(&mut std::io::Cursor::new(&mut png_buffer), image::ImageFormat::Png)
    .map_err(|e| e.to_string())
    .expect("aa");

  Ok(png_buffer)
}

pub fn indices(size: usize, reversed: bool) -> impl Iterator<Item = usize> {
  let range = 0..size;

  let pointer: Box<dyn Iterator<Item = _>> = match reversed {
    true => Box::new(range.rev()),
    false => Box::new(range),
  };

  pointer
}
