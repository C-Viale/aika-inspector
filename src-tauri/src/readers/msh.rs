use byteorder::{LittleEndian, ReadBytesExt};
use std::{fs::File, io::BufReader};

use crate::{
  models::msh::MshData,
  structs::{Vec2, Vec3},
};

pub async fn read_msh(file: &File) -> Result<MshData, std::io::Error> {
  let mut reader = BufReader::new(file);

  let magic = reader.read_i32::<LittleEndian>()?;
  let unused_0 = reader.read_i32::<LittleEndian>()?;
  let chunk_key = reader.read_i32::<LittleEndian>()?;
  let unknow_0 = reader.read_i32::<LittleEndian>()?;
  let chunk_size = reader.read_i32::<LittleEndian>()?;
  let unknow_1 = reader.read_i32::<LittleEndian>()?;
  let bone_count = reader.read_u32::<LittleEndian>()?;
  let vertices_count = reader.read_u32::<LittleEndian>()?;
  let triangles_position_count = reader.read_u32::<LittleEndian>()?;

  let mut i = 0;
  while i < bone_count {
    let mut j = 0;
    while j < 16 {
      // ignore bones for now
      reader.read_i32::<LittleEndian>()?;
      j += 1
    }

    i += 1;
  }

  let mut i = 0;
  while i < bone_count {
    // ignore bones for now
    reader.read_f32::<LittleEndian>()?;
    i += 1;
  }

  let mut vertices: Vec<Vec3> = Vec::with_capacity(vertices_count.try_into().unwrap_or_default());

  let mut bone_count = 0;

  let mut i = 0;
  while i < vertices_count {
    let vertex = Vec3 {
      x: reader.read_f32::<LittleEndian>()?,
      y: reader.read_f32::<LittleEndian>()?,
      z: reader.read_f32::<LittleEndian>()?,
    };

    vertices.push(vertex);

    if chunk_size == 48 {
      let weight = reader.read_f32::<LittleEndian>()?;
      // verticesWeights[i].weights[0] = weight;

      if weight > 0.0 {
        bone_count += 1;
      }
    }

    if chunk_size >= 44 {
      let weight = reader.read_f32::<LittleEndian>()?;
      // var weight = reader.ReadSingle();
      // verticesWeights[i].weights[1] = weight;

      if weight > 0.0 {
        bone_count += 1;
      }
    }

    if chunk_size != 36 {
      let weight = reader.read_f32::<LittleEndian>()?;
      // var weight = reader.ReadSingle();
      // verticesWeights[i].weights[2] = weight;

      if weight > 0.0 {
        bone_count += 1;
      }
    }

    let mut j = 0;
    while j < 4 {
      // verticesWeights[i].bonesId[j] = reader.ReadByte();
      reader.read_u8();

      j += 1;
    }

    // if (verticesWeights[i].bonesId[3] > 0)
    // {
    //     verticesWeights[i].weights[3] = 1.0f;
    //     boneCount++;
    // }

    reader.read_f32::<LittleEndian>()?;
    reader.read_f32::<LittleEndian>()?;
    reader.read_f32::<LittleEndian>()?;
    reader.read_f32::<LittleEndian>()?;
    reader.read_f32::<LittleEndian>()?;
    // normals[i] = new Vector3(reader.ReadSingle(), reader.ReadSingle(), reader.ReadSingle());
    // uvs[i] = new Vector2(reader.ReadSingle(), reader.ReadSingle());

    i += 1;
  }

  let triangle_count: usize = (triangles_position_count / 3).try_into().unwrap_or_default();

  let mut triangles: Vec<Vec3<i16>> = Vec::with_capacity(triangle_count);
  let mut normals: Vec<Vec3> = Vec::with_capacity(triangle_count);
  let mut uv: Vec<Vec2> = Vec::with_capacity(triangle_count);

  let mut i = 0;
  while i < triangle_count {
    triangles.push(Vec3::<i16> {
      x: reader.read_i16::<LittleEndian>()?,
      y: reader.read_i16::<LittleEndian>()?,
      z: reader.read_i16::<LittleEndian>()?,
    });
    i += 1;
  }

  Ok(MshData {
    magic,
    unused_0,
    chunk_key,
    unknow_0,
    chunk_size,
    unknow_1,
    bone_count,
    vertices_count,
    triangles_position_count,
    vertices,
    triangles,
    normals,
    uv,
  })
}
