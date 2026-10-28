use crate::{
  models::responses::{FileListItem, ModelData},
  readers::{ms3::read_ms3, msh::read_msh},
};
use std::{
  fs::{self, File},
  path::Path,
};
use tauri::ipc::{InvokeBody, Request};

pub mod models;
pub mod readers;
pub mod structs;

#[tauri::command]
fn get_filelist_from_folder(folder_path: &str) -> Result<Vec<FileListItem>, String> {
  let entries = fs::read_dir(&folder_path).map_err(|e| e.to_string())?;

  let mut files = Vec::new();

  for entry in entries {
    let entry: fs::DirEntry = entry.map_err(|e| e.to_string())?;
    let path = entry.path();

    if let Some(ext) = path.extension() {
      if ext == "msh" || ext == "MS3" {
        files.push(FileListItem {
          name: path.file_name().unwrap().to_string_lossy().into_owned(),
          path: path.to_string_lossy().into_owned(),
        });
      }
    }
  }

  Ok(files)
}

#[tauri::command]
async fn read_file(path: String) -> Result<ModelData, &'static str> {
  let Some(filename) = Path::new(&path).extension() else {
    return Err("Error while reading format");
  };

  let file = File::open(&path).expect("Unable to open file");

  if filename == "msh" {
    let msh = read_msh(&file).await.expect("Unrecognized format");
    return Ok(msh.into());
  }

  if filename == "ms3" || filename == "MS3" {
    let ms3 = read_ms3(&file).await.expect("Unrecognized format");
    return Ok(ms3.into());
  }

  Err("Unknow file format")
}

#[tauri::command]
fn write_file(request: Request<'_>) -> Result<(), String> {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
      get_filelist_from_folder,
      read_file,
      write_file
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
