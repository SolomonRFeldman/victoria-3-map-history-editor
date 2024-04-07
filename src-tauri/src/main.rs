// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod main_menu;
mod game_folder;
mod dds_to_png;

use tauri::{App, Manager};
use main_menu::MainMenu;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.maximize().unwrap();

            initialize_app_dir(app);
            Ok(())
        })
        .menu(MainMenu::create_menu())
        .on_menu_event(MainMenu::handler)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn initialize_app_dir(app: &mut App) {
  std::fs::create_dir_all(app.path_resolver().app_cache_dir().unwrap()).unwrap();
}
