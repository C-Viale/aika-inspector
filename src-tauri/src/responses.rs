use serde::Serialize;

use crate::structs::{Vec2, Vec3};

#[derive(Serialize)]
pub struct FolderData {
  pub name: String,
  pub path: String,
  pub subfolders: Vec<FolderData>,
  pub files: Vec<String>,
}

#[derive(Serialize)]
pub struct MeshData {
  pub vertices: Vec<Vec3>,
  pub triangles: Vec<Vec3<i16>>,
  pub normals: Vec<Vec3>,
  pub uv: Vec<Vec2>,
  pub bones: Vec<Bone>,
  pub bone_weights: Vec<BoneWeight>,
}

#[derive(Serialize)]
pub struct Bone {
  pub id: u32,
  pub transform: [f32; 16],
}

#[derive(Serialize)]
pub struct BoneWeight {
  pub bones_ids: [u8; 4],
  pub weights: [f32; 4],
}

#[derive(Serialize)]
pub struct TextureData {
  pub magic: String,
  pub compression_type: String,
  pub width: u32,
  pub height: u32,
  pub buffer: Vec<u8>,
}

#[derive(Serialize)]
pub struct AudioData {
  pub compression: String,
  pub buffer: Vec<u8>,
}

#[derive(Serialize)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum ReadFileResponse {
  Mesh(MeshData),
  Texture(TextureData),
  Audio(AudioData),
  Binary(Vec<u8>),
}
