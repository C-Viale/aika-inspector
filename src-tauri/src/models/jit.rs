use crate::models::responses::TextureData;

pub struct JitData {
  pub magic: String,
  pub compression_type: String,
  pub width: u32,
  pub height: u32,
  pub buffer: Vec<u8>,
}

impl Into<TextureData> for JitData {
  fn into(self) -> TextureData {
    TextureData {
      magic: self.magic,
      compression_type: self.compression_type,
      width: self.width,
      height: self.height,
      buffer: self.buffer,
    }
  }
}
