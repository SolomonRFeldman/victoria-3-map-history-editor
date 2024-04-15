use std::{collections::HashMap, path::PathBuf, sync::{Arc, Mutex}, time::Instant};
use geo::{BooleanOps, MapCoords, MultiPolygon, Polygon};
use image_dds::image::Rgba;
use serde_json::{to_vec, Value};
use tauri::{Manager, WindowMenuEvent};
use crate::{dds_to_png::DdsToPng, pdx_script_parser::parse_script, province_map_to_geojson::province_map_to_geojson};
use rayon::prelude::*;
use serde_json::Value as JsonValue;
use serde::{Serialize, Deserialize};

const FLATMAP_PATH: &str = "game/dlc/dlc004_voice_of_the_people/gfx/map/textures/flatmap_votp.dds";
const LAND_MASK_PATH: &str = "game/gfx/map/textures/land_mask.dds";
const FLATMAP_OVERLAY_PATH: &str = "game/dlc/dlc004_voice_of_the_people/gfx/map/textures/flatmap_overlay_votp.dds";
const PROVINCE_PATH: &str = "game/map_data/provinces.png";
const STATES_PATH: &str = "game/common/history/states/00_states.txt";

pub struct GameFolder {
  pub folder_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SubState {
  provinces: Vec<Value>,
  owner: String,
  coordinates: Vec<Vec<(f32, f32)>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct State {
  name: String,
  sub_states: Vec<SubState>
}

fn get_sub_states_from_state(state: &Vec<JsonValue>) -> State {
  let sub_states = state[1].as_array().unwrap().iter().filter(|item| item[0] == "create_state").map(|sub_state| {
    let sub_state_data = sub_state[1].as_array().unwrap();
    let sub_state_provinces = sub_state_data.iter().find(|item| item[0] == "owned_provinces").unwrap().as_array().unwrap()[1].as_array().unwrap();
    let sub_state_owner = sub_state_data.iter().find(|item| item[0] == "country").unwrap().as_array().unwrap()[1].as_str().unwrap();

    SubState {
      provinces: sub_state_provinces.to_vec(),
      owner: sub_state_owner.to_string(),
      coordinates: vec![]
    }
  }).collect::<Vec<SubState>>();
  State {
    name: state[0].as_str().unwrap().to_string(),
    sub_states
  }
}

impl GameFolder {
  pub fn load(&self, event: WindowMenuEvent) {
    self.load_flatmap(&event);
    self.load_land_mask(&event);
    self.load_flatmap_overlay(&event);
    // self.load_all_states(&event);
    self.load_states(&event);
    // self.load_provinces(&event);
  }

  fn load_flatmap(&self, event: &WindowMenuEvent) {
    let dds_to_png = DdsToPng { dds_file_path: self.flatmap() };

    match dds_to_png.cache(cache_dir(event)) {
      Ok(_) => handle_send_map(event, "load-flatmap"),
      Err(_) => println!("Flatmap already in cache"),
    };
  }

  fn load_land_mask(&self, event: &WindowMenuEvent) {
    let dds_to_png = DdsToPng { dds_file_path: self.land_mask() };

    if !dds_to_png.exists_in_cache(cache_dir(event)) {
      let mut png_buffer = dds_to_png.dds_to_buffer();
      for pixel in png_buffer.pixels_mut() { if pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0 { *pixel = Rgba([0, 0, 0, 0]); } }

      dds_to_png.write_image(png_buffer, dds_to_png.png_file_path(cache_dir(event))).unwrap();
      handle_send_map(event, "load-land-mask");
    } else {
      println!("Land mask already in cache");
    }
  }

  fn load_flatmap_overlay(&self, event: &WindowMenuEvent) {
    let dds_to_png = DdsToPng { dds_file_path: self.flatmap_overlay() };

    match dds_to_png.cache(cache_dir(event)) {
      Ok(_) => handle_send_map(event, "load-flatmap-overlay"),
      Err(_) => println!("Flatmap overlay already in cache"),
    };
  }

  // fn load_provinces(&self, event: &WindowMenuEvent) {
  //   match event.window().emit("load-province-data", province_map_to_geojson(self.provinces())) {
  //     Ok(_) => println!("Sent load-province-data to frontend"),
  //     Err(e) => println!("Failed to send load-province-data to frontend: {:?}", e),
  //   }
  // }
  
  fn load_states(&self, event: &WindowMenuEvent) {
    let parsed_states = parse_script(&std::fs::read_to_string(self.states()).unwrap());
    // let connaught_state = &parsed_states[0][1].as_array().unwrap().iter().find(|state| state[0] == "s:STATE_CONNAUGHT").unwrap().as_array().unwrap();
    // // let connaught_state: &Vec<Value> = parsed_states[0][1]["s:STATE_CONNAUGHT"]["create_state"]["owned_provinces"].as_array().unwrap();
    // get_sub_states_from_state(connaught_state);

    let start = Instant::now();
    let states = parsed_states[0][1].as_array().unwrap().iter().map(|state| {
      get_sub_states_from_state(state.as_array().unwrap())
    }).collect::<Vec<_>>();
    let duration = start.elapsed();
    println!("Time elapsed in get_sub_states_from_state() is: {:?}", duration);
  //   println!("Connaught state: {:?}", connaught_state);
    let start = Instant::now();
    let provinces = province_map_to_geojson(self.provinces());
    let duration = start.elapsed();
    println!("Time elapsed in province_map_to_geojson() is: {:?}", duration);

    let start = Instant::now();
    let state_geojsons = states.iter().map(|state| {
      let sub_state_geometries = state.sub_states.iter().map(|sub_state| {
        let sub_state_borders = sub_state.provinces.iter().filter_map(|province| {
          provinces.get(province.as_str().unwrap().trim_matches('"'))
        }).collect::<Vec<_>>();
        let unioned = sub_state_borders.iter().fold(sub_state_borders[0].clone(), |acc, x| acc.union(x));
        SubState {
          provinces: sub_state.provinces.clone(),
          owner: sub_state.owner.clone(),
          coordinates: unioned.iter().map(|polygon| polygon.exterior().points().map(|point| (point.x(), point.y())).collect::<Vec<(f32, f32)>>()).collect::<Vec<Vec<(f32, f32)>>>()
        }
      }).collect::<Vec<_>>();
      State {
        name: state.name.clone(),
        sub_states: sub_state_geometries
      }
    }).collect::<Vec<State>>();
    let duration = start.elapsed();
    println!("Time elapsed in state_geojsons is: {:?}", duration);
  //   println!("Province x20E0C0 has borders: {:?}", provinces.get("x20E0C0"));
  //   // provinces.get("x20E0C0").unwrap().union(provinces.get("x20E0C1").unwrap());
  //   let connacht_borders = connaught_state.iter().map(|province| provinces.get(province.as_str().unwrap()).unwrap()).collect::<Vec<_>>();
  //   let unioned = connacht_borders.iter().fold(connacht_borders[0].clone(), |acc, x| acc.union(x));
  //   println!("Unioned: {:?}", unioned);
  //   let states: Vec<Vec<(f32, f32)>> = Vec::new();
  //   let polygons_data: Vec<Vec<(f32, f32)>> = unioned.iter().map(|polygon| {
  //     polygon.exterior().0.iter().map(|&point| (point.x, point.y)).collect()
  // }).collect();

  // let array = vec![polygons_data];

    match event.window().emit("load-state-data", state_geojsons) {
      Ok(_) => println!("Sent load-state-data to frontend"),
      Err(e) => println!("Failed to send load-state-data to frontend: {:?}", e),
    }
  }
  fn load_all_states(&self, event: &WindowMenuEvent) {
    let states_script = std::fs::read_to_string(self.states()).unwrap();
    let parsed_states = parse_script(&states_script);
    let provinces_geojson = province_map_to_geojson(self.provinces());
    let all_states = parsed_states["STATES"].as_object().unwrap();
    let all_states_vec: Vec<(&String, &Value)> = all_states.iter().collect();

    // Using a thread-safe container to store results from parallel processing
    let state_geometries = Arc::new(Mutex::new(HashMap::new()));

    println!("Starting to load all states data");
    // Parallel processing of each state to calculate geometries
    all_states_vec.par_iter().for_each(|(state_name, state_details)| {
        if let Some(create_state) = state_details.get("create_state") {
            if let Some(owned_provinces) = create_state["owned_provinces"].as_array() {
                let province_geometries: Vec<&MultiPolygon<f32>> = owned_provinces.iter()
                    .filter_map(|v| v.as_str())
                    .map(|code| code.trim_matches('"')) 
                    .filter_map(|code| provinces_geojson.get(code))
                    .collect();

                if let Some(initial) = province_geometries.first().cloned() {
                    let unioned = province_geometries.iter().skip(1).fold(initial.clone(), |acc, x| acc.union(x));

                    // Lock the mutex to safely update the shared HashMap
                    let mut geometries = state_geometries.lock().unwrap();
                    geometries.insert(state_name.to_string(), unioned);
                }
            }
        }
    });

    println!("Finished loading all states data");
    // Retrieve and unlock the data container on the main thread
    let locked_geometries = Arc::try_unwrap(state_geometries).expect("Lock still has multiple owners");
    let geometries = locked_geometries.into_inner().unwrap();

    println!("States geometries: ");
    // Convert to arrays of arrays of tuples for each state
    let states_polygons_data: HashMap<String, Vec<Vec<(f32, f32)>>> = geometries.iter()
        .map(|(name, multipolygon)| {
            let polygons_data = multipolygon.0.iter()
                .map(|polygon| polygon.exterior().0.iter().map(|point| (point.x, point.y)).collect())
                .collect();
            (name.clone(), polygons_data)
        })
        .collect();
println!("States polygons data:");
    // Emit all state geometries data together
    match event.window().emit("load-state-data", &states_polygons_data) {
        Ok(_) => println!("Sent all states data to frontend"),
        Err(e) => println!("Failed to send all states data to frontend: {:?}", e),
    }
}


  // fn load_all_states(&self, event: &WindowMenuEvent) {
  //   let states_script = std::fs::read_to_string(self.states()).unwrap();
  //   let parsed_states = parse_script(&states_script);
  //   let provinces_geojson = province_map_to_geojson(self.provinces());
  //   let all_states = parsed_states["STATES"].as_object().unwrap();

  //   let mut state_geometries: HashMap<String, MultiPolygon<f32>> = HashMap::new();

  //   for (state_name, state_details) in all_states.iter() {
  //       if let Some(create_state) = state_details.get("create_state") {
  //           if let Some(owned_provinces) = create_state["owned_provinces"].as_array() {
  //               let province_geometries: Vec<&MultiPolygon<f32>> = owned_provinces.iter()
  //                   .filter_map(|v| v.as_str())
  //                   .filter_map(|code| provinces_geojson.get(code))
  //                   .collect();

  //               if let Some(initial) = province_geometries.first().cloned() {
  //                   let unioned = province_geometries.iter().skip(1).fold(initial.clone(), |acc, x| acc.union(x));
  //                   state_geometries.insert(state_name.to_string(), unioned);
  //               }
  //           }
  //       }
  //   }

  //   // Convert to arrays of arrays of tuples for each state
  //   let states_polygons_data: HashMap<String, Vec<Vec<(f32, f32)>>> = state_geometries.iter()
  //       .map(|(name, multipolygon)| {
  //           let polygons_data = multipolygon.0.iter()
  //               .map(|polygon| polygon.exterior().0.iter().map(|point| (point.x, point.y)).collect())
  //               .collect();
  //           (name.clone(), polygons_data)
  //       })
  //       .collect();

  //   // Emit each state's geometry data
  //       match event.window().emit("load-state-data", states_polygons_data) {
  //           Ok(_) => println!("Sent load-state-data for to frontend"),
  //           Err(e) => println!("Failed to send load-state-data fo"),
  //       }
// }
  fn flatmap(&self) -> PathBuf {
    self.folder_path.join(PathBuf::from(FLATMAP_PATH))
  }

  fn land_mask(&self) -> PathBuf {
    self.folder_path.join(PathBuf::from(LAND_MASK_PATH))
  }

  fn flatmap_overlay(&self) -> PathBuf {
    self.folder_path.join(PathBuf::from(FLATMAP_OVERLAY_PATH))
  }

  fn provinces(&self) -> PathBuf {
    self.folder_path.join(PathBuf::from(PROVINCE_PATH))
  }

  fn states(&self) -> PathBuf {
    self.folder_path.join(PathBuf::from(STATES_PATH))
  }
}

fn handle_send_map(event: &WindowMenuEvent, event_id: &str) {
  match event.window().emit(event_id, true) {
    Ok(_) => println!("Sent {:?} to frontend", event_id),
    Err(e) => println!("Failed to send {:?} to frontend: {:?}", event_id, e),
  }
}

fn cache_dir(event: &WindowMenuEvent) -> PathBuf {
  event.window().app_handle().path_resolver().app_cache_dir().unwrap()
}
