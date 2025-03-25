// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod building;
mod cache_config;
mod color_converter;
mod country;
mod country_definition;
mod country_setup;
mod dds_to_png;
mod game_folder;
mod geo_converters;
mod get_countries;
mod get_state_buildings;
mod get_state_populations;
mod get_states;
mod get_uncreated_country_definitions;
mod main_menu;
mod merge_buildings;
mod merge_pops;
mod pdx_script_parser;
mod province_map_to_geojson;
mod save_as_pdx_script;
mod technology;
mod transfer_provinces;
mod transfer_state;

use building::Building;
use country::Country;
use country_definition::CountryDefinition;
use main_menu::MainMenu;
use province_map_to_geojson::Coords;
use std::{
    collections::{HashMap, HashSet},
    thread,
};
use tauri::{App, Manager, Window};
use technology::Technology;
use transfer_provinces::{transfer_province as handle_transfer_province, TransferProvinceResponse};
use transfer_state::{transfer_state as handle_transfer_state, TransferStateResponse};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn transfer_state(
    state: String,
    from_country: Country,
    to_country: Country,
    from_coords: Vec<Vec<(f32, f32)>>,
    to_coords: Vec<Vec<(f32, f32)>>,
) -> TransferStateResponse {
    handle_transfer_state(&state, from_country, to_country, from_coords, to_coords)
}
#[tauri::command]
fn transfer_province(
    state: String,
    province: String,
    from_country: Country,
    to_country: Country,
    from_coords: Vec<Vec<(f32, f32)>>,
    to_coords: Vec<Vec<(f32, f32)>>,
    province_coords: Vec<Vec<(f32, f32)>>,
) -> TransferProvinceResponse {
    handle_transfer_province(
        &state,
        &province,
        from_country,
        to_country,
        from_coords,
        to_coords,
        province_coords,
    )
}
#[tauri::command]
fn cache_state(
    window: Window,
    countries: Vec<Country>,
    states: HashMap<String, Vec<Vec<(f32, f32)>>>,
) {
    thread::spawn(move || {
        let cache_dir = window.app_handle().path().app_cache_dir().unwrap();
        std::fs::write(
            cache_dir.join("states.json"),
            serde_json::to_string(&states).unwrap(),
        )
        .unwrap();
        std::fs::write(
            cache_dir.join("countries.json"),
            serde_json::to_string(&countries).unwrap(),
        )
        .unwrap();
    });
}
#[tauri::command]
fn get_building(window: Window, name: String) -> Building {
    Building::parse_from_game_folder(window)
        .iter()
        .find(|building| building.name == name)
        .unwrap()
        .clone()
}
#[tauri::command]
fn get_buildings(window: Window) -> Vec<Building> {
    Building::parse_from_game_folder(window)
}
#[tauri::command]
fn get_technologies(window: Window) -> Vec<Technology> {
    Technology::parse_from_game_folder(window)
}
#[tauri::command]
fn get_uncreated_country_definitions(
    window: Window,
    created_tag_set: HashSet<String>,
) -> Vec<CountryDefinition> {
    get_uncreated_country_definitions::get_uncreated_country_definitions(window, created_tag_set)
}
#[tauri::command]
fn create_country(
    country_definition: CountryDefinition,
    from_country: Country,
    state: String,
    coords: Coords,
) -> TransferStateResponse {
    handle_transfer_state(
        &state,
        from_country,
        Country::new(country_definition),
        coords,
        vec![],
    )
}
#[tauri::command]
fn create_country_from_province(
    country_definition: CountryDefinition,
    from_country: Country,
    state: String,
    province: String,
    state_coords: Coords,
    province_coords: Coords,
) -> TransferProvinceResponse {
    handle_transfer_province(
        &state,
        &province,
        from_country,
        Country::new(country_definition),
        state_coords,
        vec![],
        province_coords,
    )
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.maximize().unwrap();

            initialize_app_dir(app);
            MainMenu::create_menu(app);
            Ok(())
        })
        .on_menu_event(MainMenu::handler)
        .invoke_handler(tauri::generate_handler![
            transfer_state,
            transfer_province,
            cache_state,
            get_building,
            get_buildings,
            get_uncreated_country_definitions,
            create_country,
            create_country_from_province,
            get_technologies
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn initialize_app_dir(app: &mut App) {
    std::fs::create_dir_all(app.path().app_cache_dir().unwrap()).unwrap();
}
