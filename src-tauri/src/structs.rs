use std::io::{BufReader, Read};

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

#[derive(Debug, Clone, Serialize)]
pub struct RGBA {
  pub r: u16,
  pub g: u16,
  pub b: u16,
  pub a: u16,
}

impl RGBA {
  pub fn new_empty() -> Self {
    Self { r: 0, g: 0, b: 0, a: 0 }
  }
}

pub struct ByteReader<R: Read> {
  inner: BufReader<R>,
}

impl<R: Read> ByteReader<R> {
  pub fn new(inner: R) -> Self {
    Self {
      inner: BufReader::new(inner),
    }
  }

  pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), &'static str> {
    match self.inner.read_exact(buf) {
      Ok(e) => Ok(e),
      Err(_) => Err("Unable to fill buffer"),
    }
  }

  pub fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, &'static str> {
    let mut buf = vec![0u8; size];

    match self.inner.read_exact(&mut buf) {
      Ok(_) => Ok(buf),
      Err(_) => Err("Unable to fill buffer"),
    }
  }

  pub fn read_string_exact(&mut self, size: usize) -> Result<String, &'static str> {
    let mut buf = self.read_bytes(size)?;

    match str::from_utf8(&mut buf) {
      Ok(str) => Ok(str.to_owned()),
      Err(_) => Err("Unable to parse ASCII string from buffer"),
    }
  }

  pub fn read_u16_le(&mut self) -> Result<u16, &'static str> {
    let mut buf = [0u8; 2];
    match self.inner.read_exact(&mut buf) {
      Ok(_) => Ok(u16::from_le_bytes(buf)),
      Err(_) => Err("Unable to fill buffer"),
    }
  }

  pub fn read_i16_le(&mut self) -> Result<i16, &'static str> {
    let mut buf = [0u8; 2];
    match self.inner.read_exact(&mut buf) {
      Ok(_) => Ok(i16::from_le_bytes(buf)),
      Err(_) => Err("Unable to fill buffer"),
    }
  }

  pub fn read_i32_le(&mut self) -> Result<i32, &'static str> {
    let mut buf = [0u8; 4];
    match self.inner.read_exact(&mut buf) {
      Ok(_) => Ok(i32::from_le_bytes(buf)),
      Err(_) => Err("Unable to fill buffer"),
    }
  }

  pub fn read_u32_le(&mut self) -> Result<u32, &'static str> {
    let mut buf = [0u8; 4];
    match self.inner.read_exact(&mut buf) {
      Ok(_) => Ok(u32::from_le_bytes(buf)),
      Err(_) => Err("Unable to fill buffer"),
    }
  }

  pub fn read_u64_le(&mut self) -> Result<u64, &'static str> {
    let mut buf = [0u8; 8];
    match self.inner.read_exact(&mut buf) {
      Ok(_) => Ok(u64::from_le_bytes(buf)),
      Err(_) => Err("Unable to fill buffer"),
    }
  }

  pub fn read_f32_le(&mut self) -> Result<f32, &'static str> {
    let mut buf = [0u8; 4];
    match self.inner.read_exact(&mut buf) {
      Ok(_) => Ok(f32::from_le_bytes(buf)),
      Err(_) => Err("Unable to fill buffer"),
    }
  }
}
