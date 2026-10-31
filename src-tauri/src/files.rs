use std::{
  fs,
  path::{Path, PathBuf},
};

use crate::responses::FolderData;

pub fn list_directory_children(root_path: &Path) -> Result<FolderData, &'static str> {
  let Ok(dir) = fs::read_dir(root_path) else {
    return Err("Unable to read directory");
  };

  let Some(name) = root_path.file_name() else {
    return Err("Unable to get directory info");
  };

  let mut data = FolderData {
    name: name.to_string_lossy().into_owned(),
    path: root_path.to_string_lossy().into_owned(),
    subfolders: vec![],
    files: vec![],
  };

  for entry in dir {
    let Ok(entry) = entry else {
      return Err("Error while fetching folder entry");
    };

    let path = entry.path();

    if path.is_dir() {
      data.subfolders.push(list_directory_children(&path)?);
      continue;
    }

    if is_allowed_file(&path) {
      data
        .files
        .push(path.file_name().unwrap().to_string_lossy().into_owned());
    }
  }

  data.files.sort();

  Ok(data)
}

fn is_allowed_file(path: &PathBuf) -> bool {
  match path.extension() {
    None => false,
    Some(extension) => match extension.to_str() {
      Some("msh") | Some("jit") | Some("ms3") | Some("MS3") => true,
      _ => false,
    },
  }
}
