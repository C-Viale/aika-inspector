use byteorder::{LittleEndian, ReadBytesExt};
use std::{
  fs::File,
  io::{BufReader, Read},
};

use crate::{
  models::ms3::Ms3Data,
  structs::{Vec2, Vec3},
};

/*
  ? MS3 MEMORY LAYOUT

  * Header:
  magic/version 	|    i32   | 4 bytes | 0x00 - 0x03
  unknow_1 				|    i32   | 4 bytes | 0x04 - 0x07
  unknow_2 				|    i32   | 4 bytes | 0x08 - 0x0B
  has_lightmap 		|    i32   | 4 bytes | 0x0C - 0x0F
  unknow_4 				|    i32   | 4 bytes | 0x10 - 0x13
  unknow_5 				|    i32   | 4 bytes | 0x14 - 0x17
  unknow_6 				|    i32   | 4 bytes | 0x18 - 0x1B
  texture_count 	|    i32   | 4 bytes | 0x1C - 0x1F

  * Textures (texture_count + has_lightmap):
  texture_name 		|  ASCII   | 50 bytes | 0x20 - (50 * texture quantity)

  * Padding
  unused_pad			|   none   | 2 bytes
  unknown_count   |    i32   | 4 bytes
  vertex_count    |    i32   | 4 bytes
  triangle_count  |    i32   | 4 bytes
  unknown   			|  unknown | 2 bytes
  unknown   			|  unknown | 2 bytes
  unknown   			|  unknown | 2 bytes
  unknown   			|  unknown | 2 bytes
  unknown   			|  unknown | 2 bytes

  * Layout 1
  vertices f32,f32,f32 / normals f32,f32,f32  | 4 bytes * 6 * vertex_count
  uv f32,f32 																	| 4 bytes * 2 * vertex_count
  triangles i16,i16,i16 											| 2 bytes * 3 * triangle_count

  * Layout 2
  positions f32,f32,f32 																			| 4 bytes * 3 * unknown_count
  vertice_pos_index i32 / uv f32,f32 / lightmap_uv f32, f32 	| 20 bytes * vertex_count
  triangles i16,i16,i16 																	  	| 2 bytes * 3 * triangle_count

  ? The info above is enough to render a mesh and convert to other formats
  TODO: map remaining bytes
*/

pub async fn read_ms3(file: &File) -> Result<Ms3Data, std::io::Error> {
  let mut reader = BufReader::new(file);

  let magic = reader.read_i32::<LittleEndian>()?;
  let unknow_1 = reader.read_i32::<LittleEndian>()?;
  let unknow_2 = reader.read_i32::<LittleEndian>()?;
  let has_lightmap = reader.read_i32::<LittleEndian>()?;
  let unknow_3 = reader.read_i32::<LittleEndian>()?;
  let unknow_4 = reader.read_i32::<LittleEndian>()?;
  let unknow_5 = reader.read_i32::<LittleEndian>()?;
  let texture_count = reader.read_i32::<LittleEndian>()?;

  let mut lightmap_texture_name = String::new();
  let mut texture_names: Vec<String> = vec![];

  if has_lightmap == 1 {
    let mut buffer = [0u8; 50];
    reader.read_exact(&mut buffer)?;
    let end = buffer.iter().position(|&b| b == 0).unwrap_or(50);
    let texture_name = str::from_utf8(&buffer[..end]).expect("msg");
    lightmap_texture_name.push_str(texture_name);
  }

  let mut i = 0;
  while i < texture_count {
    let mut buffer = [0u8; 50];
    reader.read_exact(&mut buffer)?;

    let end = buffer.iter().position(|&b| b == 0).unwrap_or(50);

    let texture_name = str::from_utf8(&buffer[..end]).expect("msg");
    texture_names.push(texture_name.to_owned());
    i += 1;
  }

  reader.read_u16::<LittleEndian>()?; // 2 byte padding

  let first_count = reader.read_i32::<LittleEndian>()?;
  let vertex_count = reader.read_i32::<LittleEndian>()?;
  let triangle_count = reader.read_i32::<LittleEndian>()?;

  let unknow_pad_1 = reader.read_i32::<LittleEndian>()?;
  let unknow_pad_2 = reader.read_i32::<LittleEndian>()?;
  let unknow_pad_3 = reader.read_i32::<LittleEndian>()?;
  let unknow_pad_4 = reader.read_i32::<LittleEndian>()?;
  let unknow_pad_5 = reader.read_i32::<LittleEndian>()?;

  // let mut vertices: Vec<Vec3> = Vec::with_capacity(vertex_count.try_into().unwrap());
  let mut vertices: Vec<Vec3> = Vec::new();
  let mut normals: Vec<Vec3> = Vec::new();
  let mut uv: Vec<Vec2> = Vec::new();
  let mut triangles: Vec<Vec3<i16>> = Vec::new();

  // condition is not entirely correct, need more info
  if has_lightmap == 0 {
    let mut i = 0;
    while i < vertex_count {
      vertices.push(Vec3 {
        x: reader.read_f32::<LittleEndian>()?,
        y: reader.read_f32::<LittleEndian>()?,
        z: reader.read_f32::<LittleEndian>()?,
      });
      normals.push(Vec3 {
        x: reader.read_f32::<LittleEndian>()?,
        y: reader.read_f32::<LittleEndian>()?,
        z: reader.read_f32::<LittleEndian>()?,
      });

      i += 1;
    }

    let mut i = 0;
    while i < vertex_count {
      uv.push(Vec2 {
        x: reader.read_f32::<LittleEndian>()?,
        y: reader.read_f32::<LittleEndian>()?,
      });
      i += 1;
    }

    let mut i = 0;
    while i < triangle_count {
      triangles.push(Vec3 {
        x: reader.read_i16::<LittleEndian>()?,
        y: reader.read_i16::<LittleEndian>()?,
        z: reader.read_i16::<LittleEndian>()?,
      });
      i += 1;
    }
  } else {
    let mut shared_posiitions: Vec<Vec3> = Vec::new();

    let mut i = 0;
    while i < first_count {
      shared_posiitions.push(Vec3 {
        x: reader.read_f32::<LittleEndian>()?,
        y: reader.read_f32::<LittleEndian>()?,
        z: reader.read_f32::<LittleEndian>()?,
      });
      i += 1
    }

    let mut i = 0;
    while i < vertex_count {
      let position = reader.read_i32::<LittleEndian>()?;

      let position: usize = position.try_into().unwrap();

      let pos = &shared_posiitions[position];

      // reader.read_i32::<LittleEndian>()?;

      vertices.push(pos.clone());

      // uv.push(Vec2 {
      //     x: reader.read_f32::<LittleEndian>()?,
      //     y: reader.read_f32::<LittleEndian>()?,
      // });

      reader.read_f32::<LittleEndian>()?;
      reader.read_f32::<LittleEndian>()?;

      // uv.push(Vec2 {
      //     x: reader.read_f32::<LittleEndian>()?,
      //     y: reader.read_f32::<LittleEndian>()?,
      // });

      reader.read_f32::<LittleEndian>()?;
      reader.read_f32::<LittleEndian>()?;
      i += 1
    }

    let mut i = 0;
    while i < triangle_count {
      triangles.push(Vec3 {
        x: reader.read_i16::<LittleEndian>()?,
        y: reader.read_i16::<LittleEndian>()?,
        z: reader.read_i16::<LittleEndian>()?,
      });
      i += 1;
    }

    // todo!()
  }

  Ok(Ms3Data {
    magic,
    unknow_1,
    unknow_2,
    has_lightmap,
    unknow_3,
    unknow_4,
    unknow_5,
    texture_count,
    lightmap_texture_name,
    texture_names,
    unknow_pad_1,
    unknow_pad_2,
    unknow_pad_3,
    unknow_pad_4,
    unknow_pad_5,
    vertices,
    triangles,
    normals,
    uv,
  })
}
