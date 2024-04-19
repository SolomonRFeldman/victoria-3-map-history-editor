// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod main_menu;
mod game_folder;
mod dds_to_png;
mod province_map_to_geojson;
mod pdx_script_parser;
mod get_states;
mod get_countries;
mod transfer_state;
mod transfer_provinces;
mod geo_converters;

use get_countries::Country;
use tauri::{App, Manager};
use main_menu::MainMenu;
use transfer_state::{transfer_state as handle_transfer_state, TransferStateResponse};
use transfer_provinces::{transfer_province as handle_transfer_province, TransferProvinceResponse};


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn transfer_state(state: String, from_country: Country, to_country: Country, from_coords: Vec<Vec<(f32, f32)>>, to_coords: Vec<Vec<(f32, f32)>>) -> TransferStateResponse {
  handle_transfer_state(&state, from_country, to_country, from_coords, to_coords)
}
#[tauri::command]
fn transfer_province(state: String, province: String, from_country: Country, to_country: Country, from_coords: Vec<Vec<(f32, f32)>>, to_coords: Vec<Vec<(f32, f32)>>, province_coords: Vec<Vec<(f32, f32)>>) -> TransferProvinceResponse {
  handle_transfer_province(&state, &province, from_country, to_country, from_coords, to_coords, province_coords)
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
        .invoke_handler(tauri::generate_handler![transfer_state, transfer_province])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn initialize_app_dir(app: &mut App) {
  std::fs::create_dir_all(app.path_resolver().app_cache_dir().unwrap()).unwrap();
}
