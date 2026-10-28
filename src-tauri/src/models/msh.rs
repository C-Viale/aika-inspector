use serde::{Deserialize, Serialize};

use crate::{
  models::responses::ModelData,
  structs::{Vec2, Vec3},
};

#[derive(Debug, Serialize, Deserialize)]
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

impl Into<ModelData> for MshData {
  fn into(self) -> ModelData {
    ModelData {
      vertices: self.vertices,
      triangles: self.triangles,
      normals: self.normals,
      uv: self.uv,
    }
  }
}

pub struct Bone {
  pub id: i32,
  pub transform: Vec<f32>,
}

pub struct BoneWeight {
  pub bone_count: i32,
  pub bones_ids: [i8; 4],
  pub weights: [f32; 4],
}
