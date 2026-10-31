use crate::{
  files::list_directory_children,
  parsers,
  responses::{FolderData, MeshData, TextureData},
};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};
use tauri::ipc::{InvokeBody, Request};

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

#[derive(Serialize, Deserialize)]
pub struct ActionError {
  pub error: String,
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
pub async fn read_mesh(path: &str) -> Result<MeshData, ActionError> {
  let Some(extension) = Path::new(&path).extension() else {
    return_err!("Unable to open file");
  };

  let Ok(file) = File::open(&path) else {
    return_err!("Unable to open file");
  };

  if extension == "msh" {
    let msh = parsers::msh::read(&file).await;

    match msh {
      Ok(data) => return_ok!(data),
      Err(msg) => return_err!(msg),
    }
  }

  if extension == "ms3" || extension == "MS3" {
    let ms3 = parsers::ms3::read(&file).await;

    match ms3 {
      Ok(data) => return_ok!(data),
      Err(msg) => return_err!(msg),
    }
  }

  return_err!("Unknow file format")
}

#[tauri::command]
pub async fn read_texture(path: &str) -> Result<TextureData, ActionError> {
  let Some(extension) = Path::new(&path).extension() else {
    return_err!("Unable to open file");
  };

  let Ok(file) = File::open(&path) else {
    return_err!("Unable to open file.");
  };

  if extension == "jit" {
    let jit = parsers::jit::read(&file).await;
    match jit {
      Ok(data) => return_ok!(data),
      Err(msg) => return_err!(msg),
    }
  }

  todo!()
  // Ok(())
}

#[tauri::command]
pub fn write_file(request: Request<'_>) -> Result<(), String> {
  let InvokeBody::Raw(data) = request.body() else {
    return Err("expected raw bytes".into());
  };
  let header = request
    .headers()
    .get("path")
    .ok_or("missing path header")?
    .to_str()
    .map_err(|e| e.to_string())?;
  let path = urlencoding::decode(header).map_err(|e| e.to_string())?.into_owned();
  std::fs::write(&path, data).map_err(|e| format!("{path}: {e}"))
}
