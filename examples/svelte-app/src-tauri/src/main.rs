#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri_authenticator::TauriAuthenticator;

fn main() {
    tauri::AppBuilder::new()
        .plugin(TauriAuthenticator {})
        .build()
        .run();
}
