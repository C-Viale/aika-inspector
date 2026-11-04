use image::{ImageBuffer, Rgba};
use std::fs::File;

use crate::{
  responses::TextureData,
  structs::{ByteReader, RGBA},
};

pub struct JitData {
  pub magic: String,
  pub file_type: String,
  pub compression_type: Option<String>,
  pub width: u32,
  pub height: u32,
  pub buffer: Vec<u8>,
}

impl Into<TextureData> for JitData {
  fn into(self) -> TextureData {
    TextureData {
      magic: self.magic,
      file_type: self.file_type,
      compression_type: self.compression_type,
      width: self.width,
      height: self.height,
      buffer: self.buffer,
    }
  }
}

/*
  ? -------------------------- JIT --------------------------
  * Header:
  magic       |   ASCII   | 4 bytes | 0x00 - 0x03
  width 			|    u32    | 4 bytes | 0x04 - 0x07
  height 			|    u32    | 4 bytes | 0x08 - 0x0B

  * JT31 (DXT1) block - 8 bytes
  color1	       |  u16  | 2 bytes
  color2	       |  u16  | 2 bytes
  color_indices	 |  u32  | 4 bytes

  * JT33 (DXT3) block - 16 bytes
  alpha          |  u64  | 8 bytes
  color1	       |  u16  | 2 bytes
  color2	       |  u16  | 2 bytes
  color_indices	 |  u32  | 4 bytes

  * JT35 (DXT5) block - 16 bytes
  alpha          |  u64  | 8 bytes
  color1	       |  u16  | 2 bytes
  color2	       |  u16  | 2 bytes
  color_indices	 |  u32  | 4 bytes
  -------------------------------------------------------------


  ? ---------------- JT20 (TGA, uncompressed) -----------------
  * Header
  magic                         |   ASCII   | 4 bytes | 0x00 - 0x03
  id_length                     |    u8     | 1 byte  | 0x04
  color_map_type                |    u8     | 1 byte  | 0x05
  image_type                    |    u8     | 1 byte  | 0x06
  color_map_first_entry_index   |    u16    | 2 bytes | 0x07 - 0x08
  color_map_length              |    u16    | 2 bytes | 0x09 - 0x10
  color_map_entry_size          |    u8     | 1 byte  | 0x11
  x_origin                      |    u16    | 2 bytes | 0x12 - 0x13
  y_origin                      |    u16    | 2 bytes | 0x14 - 0x15
  width                         |    u16    | 2 bytes | 0x16 - 0x17
  height                        |    u16    | 2 bytes | 0x18 - 0x19
  pixel_depth                   |    u8     | 1 byte  | 0x20
  image_descriptor              |    u8     | 1 byte  | 0x21

  * Pixel block (pixel_depth = 24)
  color_chanel_1 (R or B)     |  u8  | 1 byte
  color_chanel_2 (G)          |  u8  | 1 byte
  color_chanel_3 (B or R)     |  u8  | 1 byte

  * Pixel block (pixel_depth = 32)
  color_chanel_1 (R or A)     |  u8  | 1 byte
  color_chanel_2 (G or B)     |  u8  | 1 byte
  color_chanel_3 (B or G)     |  u8  | 1 byte
  color_chanel_4 (A or R)     |  u8  | 1 byte


*/

pub async fn read(file: &File) -> Result<JitData, String> {
  let mut reader = ByteReader::new(file);

  let magic = reader.read_string_exact(4)?;
  let (file_type, compression_type) = get_file_meta(&magic)?;

  let (pixel_buffer, width, height) = match file_type {
    "DirectX" => parse_dxt(compression_type.unwrap(), reader)?,
    "TGA" => parse_tga(reader)?,
    _ => panic!(),
  };

  Ok(JitData {
    magic,
    file_type: file_type.to_owned(),
    compression_type: match compression_type {
      None => None,
      Some(str) => Some(str.to_owned()),
    },
    width,
    height,
    buffer: generate_png(&pixel_buffer, width, height)?,
  })
}

fn parse_tga(mut reader: ByteReader<&File>) -> Result<(Vec<RGBA>, u32, u32), String> {
  let id_length = reader.read_u8()?;
  let color_map_type = reader.read_u8()?;
  let image_type = reader.read_u8()?;
  let color_map_first_entry_index = reader.read_u16_le()?;
  let color_map_length = reader.read_u16_le()?;
  let color_map_entry_size = reader.read_u8()?;
  let x_origin = reader.read_u16_le()?;
  let y_origin = reader.read_u16_le()?;
  let width = reader.read_u16_le()?;
  let height = reader.read_u16_le()?;
  let pixel_depth = reader.read_u8()?;

  let image_descriptor = reader.read_u8()?;

  let top_to_bottom = (image_descriptor >> 3) & 0x01 == 1;
  let right_to_left = (image_descriptor >> 4) & 0x01 == 1;

  let width_us = width as usize;
  let height_us = width_us as usize;

  let mut pixel_buffer: Vec<RGBA> = vec![RGBA::new_empty(); width_us * height_us];

  if top_to_bottom {
    for row in 0..height_us {
      if right_to_left {
        for col in (0..width_us).rev() {
          let i = (row * width_us) + col;

          if pixel_depth == 32 {
            pixel_buffer[i].a = reader.read_u8()? as u16;
          } else {
            pixel_buffer[i].a = 255
          }

          pixel_buffer[i].r = reader.read_u8()? as u16;
          pixel_buffer[i].g = reader.read_u8()? as u16;
          pixel_buffer[i].b = reader.read_u8()? as u16;
        }
      } else {
        for col in 0..width_us {
          let i = (row * width_us) + col;

          if pixel_depth == 32 {
            pixel_buffer[i].a = reader.read_u8()? as u16;
          } else {
            pixel_buffer[i].a = 255
          }

          pixel_buffer[i].r = reader.read_u8()? as u16;
          pixel_buffer[i].g = reader.read_u8()? as u16;
          pixel_buffer[i].b = reader.read_u8()? as u16;
        }
      }
    }
  } else {
    for row in (0..height_us).rev() {
      if right_to_left {
        for col in (0..width_us).rev() {
          let i = (row * width_us) + col;

          if pixel_depth == 32 {
            pixel_buffer[i].a = reader.read_u8()? as u16;
          } else {
            pixel_buffer[i].a = 255
          }

          pixel_buffer[i].r = reader.read_u8()? as u16;
          pixel_buffer[i].g = reader.read_u8()? as u16;
          pixel_buffer[i].b = reader.read_u8()? as u16;
        }
      } else {
        for col in 0..width_us {
          let i = (row * width_us) + col;

          if pixel_depth == 32 {
            pixel_buffer[i].a = reader.read_u8()? as u16;
          } else {
            pixel_buffer[i].a = 255
          }

          pixel_buffer[i].r = reader.read_u8()? as u16;
          pixel_buffer[i].g = reader.read_u8()? as u16;
          pixel_buffer[i].b = reader.read_u8()? as u16;
        }
      }
    }
  }

  Ok((pixel_buffer, width as u32, height as u32))
}

fn parse_dxt(compression_type: &str, mut reader: ByteReader<&File>) -> Result<(Vec<RGBA>, u32, u32), String> {
  let width = reader.read_u32_le()?;
  let height = reader.read_u32_le()?;
  let block_size = get_block_size(&compression_type);
  let blocks_x = (width).div_ceil(4);
  let blocks_y = (height).div_ceil(4);

  let mut pixel_buffer: Vec<RGBA> = vec![RGBA::new_empty(); (width * height) as usize];

  for by in 0..blocks_y {
    for bx in 0..blocks_x {
      let block = reader.read_bytes(block_size as usize)?;
      let mut block_reader = ByteReader::new(&*block);

      let alpha_bits = match compression_type {
        "DXT1" => 0_u64,
        _ => block_reader.read_u64_le()?,
      };

      let color1 = block_reader.read_u16_le()?;
      let color2 = block_reader.read_u16_le()?;

      let mut pallete: [RGBA; 4] = [
        parse_rgb565(color1),
        parse_rgb565(color2),
        RGBA { r: 0, g: 0, b: 0, a: 0 },
        RGBA { r: 0, g: 0, b: 0, a: 0 },
      ];

      if color1 > color2 || compression_type != "DXT1" {
        pallete[2].r = (2 * pallete[0].r + pallete[1].r) / 3;
        pallete[2].g = (2 * pallete[0].g + pallete[1].g) / 3;
        pallete[2].b = (2 * pallete[0].b + pallete[1].b) / 3;
        pallete[2].a = 255;

        pallete[3].r = (pallete[0].r + 2 * pallete[1].r) / 3;
        pallete[3].g = (pallete[0].g + 2 * pallete[1].g) / 3;
        pallete[3].b = (pallete[0].b + 2 * pallete[1].b) / 3;
        pallete[3].a = 255;
      } else {
        pallete[2].r = (pallete[0].r + pallete[1].r) / 2;
        pallete[2].g = (pallete[0].g + pallete[1].g) / 2;
        pallete[2].b = (pallete[0].b + pallete[1].b) / 2;
        pallete[2].a = 255;
      }

      let color_indices = block_reader.read_u32_le()?;

      let mut dxt5_alpha_table = [0, 0, 0, 0, 0, 0, 0, 0];
      let mut dxt5_alpha_bits = 0_u64;

      if compression_type == "DXT5" {
        let alpha_bytes = alpha_bits.to_le_bytes();

        let a0 = alpha_bytes[0];
        let a1 = alpha_bytes[1];

        dxt5_alpha_table[0] = a0;
        dxt5_alpha_table[1] = a1;

        if a0 > a1 {
          for i in 1..7 {
            dxt5_alpha_table[i + 1] = (((7 - i) * a0 as usize + i * a1 as usize) / 7) as u8;
          }
        } else {
          for i in 1..5 {
            dxt5_alpha_table[i + 1] = (((5 - i) * a0 as usize + i * a1 as usize) / 5) as u8
          }
          dxt5_alpha_table[6] = 0;
          dxt5_alpha_table[7] = 255;
        }

        for i in 0..6 {
          dxt5_alpha_bits |= ((alpha_bytes[2 + i] as u64) << (8 * i)) as u64
        }
      }

      for i in 0..16 {
        let index = color_indices >> (i * 2) & 3;

        let alpha: u16 = match compression_type {
          "DXT1" => 255,
          "DXT3" => (((alpha_bits >> (i * 4)) & 0xF) * 17) as u16,
          "DXT5" => dxt5_alpha_table[(((dxt5_alpha_bits) >> (i * 3)) & 7) as usize] as u16,
          _ => 255,
        };

        let x = bx * 4 + i % 4;
        let y = by * 4 + i / 4;

        if x >= width || y >= height {
          continue;
        }

        let dst: usize = (y * width + x) as usize;
        let src: usize = (index) as usize;

        pixel_buffer[dst].r = pallete[src].r;
        pixel_buffer[dst].g = pallete[src].g;
        pixel_buffer[dst].b = pallete[src].b;
        pixel_buffer[dst].a = alpha;
      }
    }
  }

  Ok((pixel_buffer, width, height))
}

fn get_file_meta(magic: &str) -> Result<(&'static str, Option<&'static str>), String> {
  match magic {
    "JT31" => Ok(("DirectX", Some("DXT1"))),
    "JT33" => Ok(("DirectX", Some("DXT3"))),
    "JT35" => Ok(("DirectX", Some("DXT5"))),
    "JT20" => Ok(("TGA", None)),
    _ => Err("Unknown magic".to_owned()),
  }
}

fn get_block_size(compression: &str) -> u8 {
  match compression {
    "DXT1" => 8,
    "DXT3" => 16,
    "DXT5" => 16,
    _ => unreachable!(),
  }
}

fn parse_rgb565(value: u16) -> RGBA {
  let red = (value >> 11) & 31;
  let green = (value >> 5) & 63;
  let blue = value & 31;

  RGBA {
    r: (red << 3) | (red >> 2),
    g: (green << 2) | (green >> 4),
    b: (blue << 3) | (blue >> 2),
    a: 255,
  }
}

pub fn generate_png(pixels: &[RGBA], width: u32, height: u32) -> Result<Vec<u8>, &'static str> {
  if pixels.len() != (width * height) as usize {
    return Err("Pixel count does not match image dimensions");
  }

  let raw_pixels: Vec<u8> = pixels
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
