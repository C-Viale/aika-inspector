use std::fs::File;

use crate::{
  builder::{build_png, indices},
  responses::TextureData,
  structs::{ByteReader, RGBA},
};

/*
  ? TGA byte layout
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
  blue     |  u8  | 1 byte
  green    |  u8  | 1 byte
  red      |  u8  | 1 byte

  * Pixel block (pixel_depth = 32)
  blue     |  u8  | 1 byte
  green    |  u8  | 1 byte
  red      |  u8  | 1 byte
  alpha    |  u8  | 1 byte
*/

pub fn read(file: &File) -> Result<TextureData, String> {
  let reader = ByteReader::new(file);

  let (pixel_buffer, width, height) = parse(reader)?;

  Ok(TextureData {
    magic: None,
    file_type: "TGA".to_owned(),
    compression_type: None,
    width,
    height,
    buffer: build_png(&pixel_buffer, width, height)?,
  })
}

pub fn parse(mut reader: ByteReader<&File>) -> Result<(Vec<RGBA>, u32, u32), String> {
  let _id_length = reader.read_u8()?;
  let _color_map_type = reader.read_u8()?;
  let _image_type = reader.read_u8()?;

  let _color_map_first_entry_index = reader.read_u16_le()?;
  let _color_map_length = reader.read_u16_le()?;
  let _color_map_entry_size = reader.read_u8()?;

  let _x_origin = reader.read_u16_le()?;
  let _y_origin = reader.read_u16_le()?;

  let width = reader.read_u16_le()?;
  let height = reader.read_u16_le()?;
  let pixel_depth = reader.read_u8()?;

  let image_descriptor = reader.read_u8()?;

  /*
   * descriptor
   0 - 3 | alpha bits per pixel | 0x0f
   4     | right to left        | 0x10
   5     | top to botton        | 0x20
   6 - 7 | unused               | 0xc0
  */

  let top_to_bottom = image_descriptor & 0x20 == 1;
  let right_to_left = image_descriptor & 0x10 == 1;

  let width_us = width as usize;
  let height_us = height as usize;

  let mut pixel_buffer: Vec<RGBA> = vec![RGBA::new_empty(); width_us * height_us];

  for row in indices(height_us, !top_to_bottom) {
    for col in indices(width_us, right_to_left) {
      let i = (row * width_us) + col;

      pixel_buffer[i].b = reader.read_u8()? as u16;
      pixel_buffer[i].g = reader.read_u8()? as u16;

      // some files are missing 16 bytes at the end
      // so we are padding with zeros
      pixel_buffer[i].r = reader.read_u8().unwrap_or(0) as u16;

      if pixel_depth == 32 {
        pixel_buffer[i].a = reader.read_u8().unwrap_or(0) as u16;
      } else {
        pixel_buffer[i].a = 255
      }
    }
  }

  Ok((pixel_buffer, width as u32, height as u32))
}
