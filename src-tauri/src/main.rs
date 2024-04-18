// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod main_menu;
mod game_folder;
mod dds_to_png;
mod province_map_to_geojson;
mod pdx_script_parser;
mod get_states;
mod get_countries;

use std::collections::HashMap;
use geo::{BooleanOps, LineString, MultiPolygon, Polygon};
use get_countries::{Country, State};
use serde::Serialize;
use tauri::{App, Manager};
use main_menu::MainMenu;

#[derive(Serialize)]
struct TransferStateResponse {
  to_country: Country,
  from_country: Country,
  state_coords: Vec<Vec<(f32, f32)>>,
}

fn multi_poly_to_vec(multi_poly: MultiPolygon<f32>) -> Vec<Vec<(f32, f32)>> {
  let mut exterior_coords = multi_poly.clone().into_iter().map(|poly| { line_string_to_vec(poly.exterior().clone()) }).collect::<Vec<Vec<(f32, f32)>>>();
  let interior_coords = multi_poly.clone().into_iter().map(|poly| { poly.interiors().iter().map(|line_string| { line_string_to_vec(line_string.clone()) }).collect::<Vec<Vec<(f32, f32)>>>() }).collect::<Vec<Vec<Vec<(f32, f32)>>>>();
  exterior_coords.extend(interior_coords.into_iter().flatten());
  exterior_coords
}

fn line_string_to_vec(line_string: LineString<f32>) -> Vec<(f32, f32)> {
  line_string.into_iter().map(|point| { (point.x, point.y) }).collect()
}

fn vec_to_multi_poly(coords: Vec<Vec<(f32, f32)>>) -> MultiPolygon<f32> {
  coords.into_iter().map(|coords| { Polygon::new(LineString::from(coords), vec![]) }).collect()
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn transfer_state(state: &str, from_country: Country, to_country: Country, from_coords: Vec<Vec<(f32, f32)>>, to_coords: Vec<Vec<(f32, f32)>>) -> TransferStateResponse {
  println!("State: {}", state);
  println!("From Country: {}", from_country.name);
  println!("To Country: {}", to_country.name);
  println!("From Coords: {:?}", from_coords);
  println!("To Coords: {:?}", to_coords);
  let start = std::time::Instant::now();
  let from_multi_poly: MultiPolygon<f32> = vec_to_multi_poly(from_coords);
  let to_multi_poly: MultiPolygon<f32> = vec_to_multi_poly(to_coords);
  let union = from_multi_poly.union(&to_multi_poly);
  let new_state_coords = multi_poly_to_vec(union.clone());

  let new_from_country_coords = multi_poly_to_vec(vec_to_multi_poly(from_country.coordinates.clone()).difference(&union));
  let mut new_from_country = from_country.clone();
  new_from_country.states = from_country.states.iter().filter(|from_state| from_state.name != state).cloned().collect();
  new_from_country.coordinates = new_from_country_coords;
  

  let new_to_country_coords = multi_poly_to_vec(vec_to_multi_poly(to_country.coordinates.clone()).union(&union));
  let mut new_to_country = to_country.clone();
  let existing_state = new_to_country.states.iter().find(|to_state| to_state.name == state);
  new_to_country.coordinates = new_to_country_coords;
  match existing_state {
    Some(to_state) => {
      let mut new_provinces = to_state.provinces.clone();
      new_provinces.extend(from_country.states.iter().find(|from_state| from_state.name == state).unwrap().provinces.clone());
      new_to_country.states = new_to_country.states.iter().filter(|to_state| to_state.name != state).cloned().collect();
      new_to_country.states.push(State {
        name: state.to_string(),
        provinces: new_provinces,
      });
    },
    None => {
      new_to_country.states.push(from_country.states.iter().find(|from_state| from_state.name == state).unwrap().clone());
    },
  }
  println!("Time: {:?}", start.elapsed());
  TransferStateResponse {
    to_country: new_to_country,
    from_country: new_from_country,
    state_coords: new_state_coords,
  }
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
        .invoke_handler(tauri::generate_handler![transfer_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn initialize_app_dir(app: &mut App) {
  std::fs::create_dir_all(app.path_resolver().app_cache_dir().unwrap()).unwrap();
}
