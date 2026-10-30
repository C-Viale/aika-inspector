pub mod actions;
pub(crate) mod files;
pub(crate) mod models;
pub(crate) mod readers;
pub(crate) mod structs;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
      actions::get_folder_structure,
      actions::read_texture,
      actions::read_mesh,
      actions::write_file
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
