use std::{fs::File, vec};

use crate::{
  responses::{Bone, BoneWeight, MeshData},
  structs::{ByteReader, Vec2, Vec3},
};
#[allow(dead_code)]
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
  pub bones: Vec<MshBone>,
  pub bone_weights: Vec<MshBoneWeight>,
}

impl Into<MeshData> for MshData {
  fn into(self) -> MeshData {
    let mut bones = vec![];
    for i in self.bones {
      bones.push(i.into());
    }

    let mut bone_weights = vec![];

    for i in self.bone_weights {
      bone_weights.push(i.into())
    }

    MeshData {
      vertices: self.vertices,
      triangles: self.triangles,
      normals: self.normals,
      uv: self.uv,
      bones,
      bone_weights,
    }
  }
}

pub struct MshBone {
  pub id: u32,
  pub transform: [f32; 16],
}

impl MshBone {
  pub fn new_empty() -> Self {
    Self {
      id: 0,
      transform: [0_f32; 16],
    }
  }
}

impl Into<Bone> for MshBone {
  fn into(self) -> Bone {
    Bone {
      id: self.id,
      transform: self.transform,
    }
  }
}

pub struct MshBoneWeight {
  // pub bone_count: i32,
  pub bones_ids: [u8; 4],
  pub weights: [f32; 4],
}

impl Into<BoneWeight> for MshBoneWeight {
  fn into(self) -> BoneWeight {
    BoneWeight {
      bones_ids: self.bones_ids,
      weights: self.weights,
    }
  }
}

pub fn read(file: &File) -> Result<MshData, &'static str> {
  let mut reader = ByteReader::new(file);

  let magic = reader.read_i32_le()?;
  let unused_0 = reader.read_i32_le()?;
  let chunk_key = reader.read_i32_le()?;
  let unknow_0 = reader.read_i32_le()?;
  let chunk_size = reader.read_i32_le()?;
  let unknow_1 = reader.read_i32_le()?;
  let bone_count = reader.read_u32_le()?;
  let vertices_count = reader.read_u32_le()?;
  let triangles_position_count = reader.read_u32_le()?;

  let mut bones = Vec::with_capacity(bone_count as usize);

  for _ in 0..bone_count {
    let mut bone = MshBone::new_empty();

    for i in 0..16 {
      bone.transform[i] = reader.read_f32_le()?;
    }

    bones.push(bone);
  }

  for i in 0..bone_count {
    bones[i as usize].id = reader.read_u32_le()?;
  }

  let mut vertices: Vec<Vec3> = Vec::with_capacity(vertices_count as usize);
  let mut normals: Vec<Vec3> = Vec::with_capacity(vertices_count as usize);
  let mut uv: Vec<Vec2> = Vec::with_capacity(vertices_count as usize);
  let mut bone_weights: Vec<MshBoneWeight> = Vec::with_capacity(vertices_count as usize);
  let mut bone_count = 0;

  // let mut vertex_weight_joints_count = 0;

  for _ in 0..vertices_count {
    let vertex = Vec3 {
      x: reader.read_f32_le()?,
      y: reader.read_f32_le()?,
      z: reader.read_f32_le()?,
    };

    let mut bone_weight = MshBoneWeight {
      // bone_count: 0,
      bones_ids: [0_u8; 4],
      weights: [0_f32; 4],
    };

    vertices.push(vertex);

    if chunk_size == 48 {
      let weight = reader.read_f32_le()?;
      bone_weight.weights[0] = weight;

      if weight > 0.0 {
        bone_count += 1;
      }
    }

    if chunk_size >= 44 {
      let weight = reader.read_f32_le()?;
      bone_weight.weights[1] = weight;

      if weight > 0.0 {
        bone_count += 1;
      }
    }

    if chunk_size != 36 {
      let weight = reader.read_f32_le()?;
      bone_weight.weights[2] = weight;

      if weight > 0.0 {
        bone_count += 1;
      }
    }

    for j in 0..4 {
      bone_weight.bones_ids[j] = reader.read_u8()?;
    }

    if bone_weight.bones_ids[3] > 0 {
      bone_weight.weights[3] = 1.0;
      bone_count += 1
    }

    // vertex_weight_joints_count += bone_count;

    normals.push(Vec3 {
      x: reader.read_f32_le()?,
      y: reader.read_f32_le()?,
      z: reader.read_f32_le()?,
    });
    uv.push(Vec2 {
      x: reader.read_f32_le()?,
      y: reader.read_f32_le()? - 1.0,
    });
    bone_weights.push(bone_weight);
  }

  let triangle_count: usize = (triangles_position_count / 3).try_into().unwrap_or_default();

  let mut triangles: Vec<Vec3<i16>> = Vec::with_capacity(triangle_count);

  for _ in 0..triangle_count {
    triangles.push(Vec3::<i16> {
      x: reader.read_i16_le()?,
      y: reader.read_i16_le()?,
      z: reader.read_i16_le()?,
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
    bones,
    bone_weights,
  })
}
