use serde::{Deserialize, Serialize};

use crate::structs::{Vec2, Vec3};

#[derive(Serialize, Deserialize)]
pub struct FileListItem {
  pub name: String,
  pub path: String,
}

#[derive(Serialize, Deserialize)]
pub struct ModelData {
  pub vertices: Vec<Vec3>,
  pub triangles: Vec<Vec3<i16>>,
  pub normals: Vec<Vec3>,
  pub uv: Vec<Vec2>,
}
