#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri_authenticator::TauriAuthenticator;

fn main() {
  tauri::Builder::default()
    .plugin(TauriAuthenticator::default())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
