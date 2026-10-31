use byteorder::{LittleEndian, ReadBytesExt};
use std::{fs::File, io::BufReader};

use crate::{
  responses::MeshData,
  structs::{Vec2, Vec3},
};

pub struct MshData {
  pub magic: i32,
  pub unused_0: i32,
  pub chunk_key: i32,
  pub unknow_0: i32,
  pub chunk_size: i32,
  pub unknow_1: i32,
  pub bone_count: u32,
  pub vertices_count: u32,
  pub triangles_position_count: u32,

  pub vertices: Vec<Vec3>,
  pub triangles: Vec<Vec3<i16>>,
  pub normals: Vec<Vec3>,
  pub uv: Vec<Vec2>,
}

impl Into<MeshData> for MshData {
  fn into(self) -> MeshData {
    MeshData {
      vertices: self.vertices,
      triangles: self.triangles,
      normals: self.normals,
      uv: self.uv,
    }
  }
}

pub struct MshBone {
  pub id: i32,
  pub transform: Vec<f32>,
}

pub struct MshBoneWeight {
  pub bone_count: i32,
  pub bones_ids: [i8; 4],
  pub weights: [f32; 4],
}

pub async fn read(file: &File) -> Result<MshData, std::io::Error> {
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

  for _ in 0..bone_count {
    for _ in 0..16 {
      // ignore bones for now
      reader.read_i32::<LittleEndian>()?;
    }
  }

  for _ in 0..bone_count {
    // ignore bones for now
    reader.read_f32::<LittleEndian>()?;
  }

  let mut vertices: Vec<Vec3> = Vec::with_capacity(vertices_count.try_into().unwrap_or_default());

  let mut bone_count = 0;

  for _ in 0..vertices_count {
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

    for _ in 0..4 {
      // verticesWeights[i].bonesId[j] = reader.ReadByte();
      reader.read_u8()?;
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
  }

  let triangle_count: usize = (triangles_position_count / 3).try_into().unwrap_or_default();

  let mut triangles: Vec<Vec3<i16>> = Vec::with_capacity(triangle_count);
  let normals: Vec<Vec3> = Vec::with_capacity(triangle_count);
  let uv: Vec<Vec2> = Vec::with_capacity(triangle_count);

  for _ in 0..triangle_count {
    triangles.push(Vec3::<i16> {
      x: reader.read_i16::<LittleEndian>()?,
      y: reader.read_i16::<LittleEndian>()?,
      z: reader.read_i16::<LittleEndian>()?,
    });
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
