use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vec2<T = f32> {
  pub x: T,
  pub y: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vec3<T = f32> {
  pub x: T,
  pub y: T,
  pub z: T,
}
