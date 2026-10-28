use serde::{Deserialize, Serialize};

use crate::{
  models::responses::ModelData,
  structs::{Vec2, Vec3},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ms3Data {
  pub magic: i32,
  pub unknow_1: i32,
  pub unknow_2: i32,
  pub has_lightmap: i32,
  pub unknow_3: i32,
  pub unknow_4: i32,
  pub unknow_5: i32,
  pub texture_count: i32,
  pub lightmap_texture_name: String,
  pub texture_names: Vec<String>,

  pub unknow_pad_1: i32,
  pub unknow_pad_2: i32,
  pub unknow_pad_3: i32,
  pub unknow_pad_4: i32,
  pub unknow_pad_5: i32,

  pub vertices: Vec<Vec3>,
  pub triangles: Vec<Vec3<i16>>,
  pub normals: Vec<Vec3>,
  pub uv: Vec<Vec2>,
}

impl Into<ModelData> for Ms3Data {
  fn into(self) -> ModelData {
    ModelData {
      vertices: self.vertices,
      triangles: self.triangles,
      normals: self.normals,
      uv: self.uv,
    }
  }
}
