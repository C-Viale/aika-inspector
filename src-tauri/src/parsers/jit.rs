use std::fs::File;

use crate::{
  builder::build_png,
  parsers::tga,
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
      magic: Some(self.magic),
      file_type: self.file_type,
      compression_type: self.compression_type,
      width: self.width,
      height: self.height,
      buffer: self.buffer,
    }
  }
}

/*
  ? JIT byte layout
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

  * JT20 (TGA) - see tga.rs

  ! JT41 - unknown
*/

pub fn read(file: &File) -> Result<JitData, String> {
  let mut reader = ByteReader::new(file);

  let magic = reader.read_string_exact(4)?;
  let (file_type, compression_type) = get_file_meta(&magic)?;

  let (pixel_buffer, width, height) = match file_type {
    "DirectX" => parse_dxt(compression_type.unwrap(), reader)?,
    "TGA" => tga::parse(reader)?,
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
    buffer: build_png(&pixel_buffer, width, height)?,
  })
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
    "JT31" | "JT41" => Ok(("DirectX", Some("DXT1"))),
    "JT33" => Ok(("DirectX", Some("DXT3"))),
    "JT35" => Ok(("DirectX", Some("DXT5"))),
    "JT20" => Ok(("TGA", None)),
    m => Err(format!("Unknown magic: {}", m)),
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
