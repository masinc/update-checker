#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod scoop;
mod update_checker;

pub use update_checker::{UpdateChecker, UpdateStatus};

use crate::scoop::ScoopUpdateChecker;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn check_update(name: &str) -> Result<Vec<UpdateStatus>, String> {
    match name {
        "scoop" => ScoopUpdateChecker::default()
            .check_update()
            .await
            .map_err(|e| e.to_string()),

        _ => Err(format!("{name} is not found")),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, check_update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
