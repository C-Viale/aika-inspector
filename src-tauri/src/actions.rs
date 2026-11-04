use std::{
  fs::{self, File},
  path::Path,
};

use crate::{
  files::list_directory_children,
  parsers,
  responses::{ActionError, AudioData, FolderData, ReadFileResponse},
};

macro_rules! return_err {
  ($msg:expr) => {
    return Err(ActionError {
      error: $msg.to_string(),
    })
  };
}

macro_rules! return_ok {
  ($data:expr) => {
    return Ok($data.into())
  };
}

#[tauri::command]
pub async fn get_folder_structure(path: &str) -> Result<FolderData, ActionError> {
  let result = list_directory_children(Path::new(path));

  match result {
    Ok(data) => return_ok!(data),
    Err(msg) => return_err!(msg),
  }
}

#[tauri::command]
pub async fn read_file(path: &str) -> Result<ReadFileResponse, ActionError> {
  let Some(extension) = Path::new(&path).extension() else {
    return_err!("Unable to open file");
  };

  let Ok(file) = File::open(&path) else {
    return_err!("Unable to open file");
  };

  if extension == "msh" {
    let msh = parsers::msh::read(&file).await;

    match msh {
      Ok(data) => return_ok!(ReadFileResponse::Mesh(data.into())),
      Err(msg) => return_err!(msg),
    }
  }

  if extension == "ms3" || extension == "MS3" {
    let ms3 = parsers::ms3::read(&file).await;

    match ms3 {
      Ok(data) => return_ok!(ReadFileResponse::Mesh(data.into())),
      Err(msg) => return_err!(msg),
    }
  }

  if extension == "jit" {
    let jit = parsers::jit::read(&file).await;
    match jit {
      Ok(data) => return_ok!(ReadFileResponse::Texture(data.into())),
      Err(msg) => return_err!(msg),
    }
  }

  if extension == "mp3" {
    let Ok(bytes) = fs::read(path) else {
      return_err!("Unable to open file");
    };

    return_ok!(ReadFileResponse::Audio(AudioData {
      compression: "mp3".to_owned(),
      buffer: bytes
    }))
  }

  if extension == "bin" {
    let Ok(bytes) = fs::read(path) else {
      return_err!("Unable to open file");
    };

    let is_mp3 = parsers::bin::is_mp3(&bytes);
    if is_mp3 {
      return_ok!(ReadFileResponse::Audio(AudioData {
        compression: "mp3".to_owned(),
        buffer: bytes
      }));
    }

    return_ok!(ReadFileResponse::Binary(bytes))
  }

  return_err!("Unknow file format")
}

#[tauri::command]
pub fn write_file(path: String, body: Vec<u8>) -> Result<(), String> {
  std::fs::write(&path, body).map_err(|e| format!("{path}: {e}"))
}
