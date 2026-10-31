use serde::{Deserialize, Serialize};

use crate::structs::{Vec2, Vec3};

#[derive(Serialize, Deserialize)]
pub struct FolderData {
  pub name: String,
  pub path: String,
  pub subfolders: Vec<FolderData>,
  pub files: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MeshData {
  pub vertices: Vec<Vec3>,
  pub triangles: Vec<Vec3<i16>>,
  pub normals: Vec<Vec3>,
  pub uv: Vec<Vec2>,
}

#[derive(Serialize, Deserialize)]
pub struct TextureData {
  pub magic: String,
  pub compression_type: String,
  pub width: u32,
  pub height: u32,
  pub buffer: Vec<u8>,
}
