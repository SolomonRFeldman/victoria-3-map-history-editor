// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod building;
mod cache_config;
mod color_converter;
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
mod legacy_country;
mod main_menu;
mod merge_states;
mod models;
mod pdx_script_parser;
mod province_map_to_geojson;
mod save_as_pdx_script;
mod technology;
mod transfer_provinces;
mod transfer_state;

use building::Building;
use country_definition::CountryDefinition;
use main_menu::MainMenu;
use models::{country, state};
use sea_orm::{
    ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait,
};
use std::collections::HashSet;
use tauri::{async_runtime::block_on, App, Manager, Window};
use technology::Technology;
use transfer_provinces::TransferProvinceResponse;
use transfer_state::TransferStateResponse;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
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
fn get_countries(window: Window) -> Vec<country::Model> {
    let db = window.state::<DatabaseConnection>().inner();
    block_on(country::Entity::find().all(db)).unwrap()
}
#[tauri::command]
fn get_states(window: Window, country_id: i32) -> Vec<state::Model> {
    let db = window.state::<DatabaseConnection>().inner();
    block_on(
        state::Entity::find()
            .filter(state::Column::CountryId.eq(country_id))
            .all(db),
    )
    .unwrap()
}
#[tauri::command]
fn transfer_state(window: Window, state_id: i32, country_id: i32) -> TransferStateResponse {
    let db = window.state::<DatabaseConnection>().inner();
    let state = block_on(state::Entity::find_by_id(state_id).one(db))
        .unwrap()
        .unwrap();
    let to_country = block_on(country::Entity::find_by_id(country_id).one(db))
        .unwrap()
        .unwrap();
    let from_country = block_on(country::Entity::find_by_id(state.country_id).one(db))
        .unwrap()
        .unwrap();
    let txn = block_on(db.begin()).unwrap();
    let resp =
        transfer_state::transfer_state(&txn, state.into(), from_country.into(), to_country.into());
    block_on(txn.commit()).unwrap();
    resp
}
#[tauri::command]
fn transfer_province(
    window: Window,
    province: String,
    province_coords: Vec<Vec<(f32, f32)>>,
    state_id: i32,
    country_id: i32,
) -> TransferProvinceResponse {
    let db = window.state::<DatabaseConnection>().inner();
    let state = block_on(state::Entity::find_by_id(state_id).one(db))
        .unwrap()
        .unwrap();
    let to_country = block_on(country::Entity::find_by_id(country_id).one(db))
        .unwrap()
        .unwrap();
    let from_country = block_on(country::Entity::find_by_id(state.country_id).one(db))
        .unwrap()
        .unwrap();
    let txn = block_on(db.begin()).unwrap();
    let resp = transfer_provinces::transfer_province(
        &txn,
        state.into(),
        from_country.into(),
        to_country.into(),
        province,
        province_coords,
    );
    block_on(txn.commit()).unwrap();
    resp
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
            initialize_db(app);
            MainMenu::create_menu(app).unwrap();
            Ok(())
        })
        .on_menu_event(MainMenu::handler)
        .invoke_handler(tauri::generate_handler![
            get_building,
            get_buildings,
            get_uncreated_country_definitions,
            get_technologies,
            get_countries,
            get_states,
            transfer_state,
            transfer_province
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn initialize_app_dir(app: &mut App) {
    std::fs::create_dir_all(app.path().app_cache_dir().unwrap()).unwrap();
}

fn initialize_db(app: &mut App) {
    let db_path = app.path().app_cache_dir().unwrap().join("database.db");
    let db_url = format!("sqlite://{}?mode=rwc", db_path.display());
    let db = block_on(Database::connect(db_url)).unwrap();

    app.manage(db);
}
