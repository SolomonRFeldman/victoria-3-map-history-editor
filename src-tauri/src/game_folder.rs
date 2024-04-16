use std::{collections::HashMap, path::PathBuf, sync::{Arc, Mutex}, time::Instant};
use geo::{BooleanOps, MapCoords, MultiPolygon, Polygon};
use image::{io::Reader as ImageReader, ImageBuffer, Rgb};
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
  provinces: Vec<String>,
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
      provinces: sub_state_provinces.iter().map(|province| province.as_str().unwrap().trim_matches('"').to_string()).filter(|province| province.len() > 6 && province.chars().next().unwrap() == 'x').collect::<Vec<String>>(),
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
    println!("Time to load states: {:?}", start.elapsed());
    
    let start = Instant::now();
    let mut color_map = HashMap::<Rgb<u8>, Rgb<u8>>::new();
    states.iter().for_each(|state| {
      state.sub_states.iter().for_each(|sub_state| {
        // let valid_provinces: Vec<String> = sub_state.provinces.iter().filter(|province| province.as_str().unwrap().trim_matches('"').len() == 7).map(|province| province.as_str().unwrap().to_string()).collect();
        if sub_state.provinces.len() == 0 {
          println!("No valid provinces for state: {:?}", state.name);
          return;
        }
        let first_province = sub_state.provinces[0].trim_matches('"');
        let red: String = first_province.chars().skip(1).take(2).collect::<String>();
        let green: String = first_province.chars().skip(3).take(2).collect::<String>();
        let blue: String = first_province.chars().skip(5).take(2).collect::<String>();

        let color_to_turn = Rgb([u8::from_str_radix(&red, 16).unwrap(), u8::from_str_radix(&green, 16).unwrap(), u8::from_str_radix(&blue, 16).unwrap()]);
        sub_state.provinces.iter().for_each(|province| {
          let red = province.chars().skip(1).take(2).collect::<String>();
          let green = province.chars().skip(3).take(2).collect::<String>();
          let blue = province.chars().skip(5).take(2).collect::<String>();

          let color = Rgb([u8::from_str_radix(&red, 16).unwrap(), u8::from_str_radix(&green, 16).unwrap(), u8::from_str_radix(&blue, 16).unwrap()]);
          color_map.insert(color, color_to_turn);
        })
      });
    });
    // println!("Color map: {:?}", color_map);
    println!("Time to load color map: {:?}", start.elapsed());
    
    let start = Instant::now();
    let mut provinces = ImageReader::open(self.provinces()).unwrap().decode().unwrap().into_rgb8();

    provinces.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
      let color = color_map.get(&pixel).unwrap_or(&Rgb([0, 0, 0]));
      *pixel = *color;
    });
    provinces.save(cache_dir(event).join("states.png")).unwrap();
    println!("Time to color provinces: {:?}", start.elapsed());
    // let arizona = states.iter().find(|state| state.name == "s:STATE_ARIZONA").unwrap();
    // println!("Arazona: {:?}", arizona);
    let start = Instant::now();
    let state_borders = province_map_to_geojson(cache_dir(event).join("states.png"));
    println!("Time to load state borders: {:?}", start.elapsed());

    let start = Instant::now();
    let states_with_coords = states.iter().map(|state| {
      let sub_states_with_coords = state.sub_states.iter().map(|sub_state| {
        let state_geometries = state_borders.get(&sub_state.provinces[0]);

        match state_geometries {
          Some(geometries) => {
            SubState {
              provinces: sub_state.provinces.clone(),
              owner: sub_state.owner.clone(),
              coordinates: geometries.to_vec()
            }
          },
          None => {
            println!("No geometries for state: {:?}", state.name);
            println!("Provinces: {:?}", sub_state.provinces);
            SubState {
              provinces: sub_state.provinces.clone(),
              owner: sub_state.owner.clone(),
              coordinates: vec![]
            }
          }
        }
      }).collect::<Vec<SubState>>();
      State {
        name: state.name.clone(),
        sub_states: sub_states_with_coords
      }
    }).collect::<Vec<State>>();
    println!("Time to load states with coords: {:?}", start.elapsed());

    match event.window().emit("load-state-data", states_with_coords) {
      Ok(_) => println!("Sent load-state-data to frontend"),
      Err(e) => println!("Failed to send load-state-data to frontend: {:?}", e),
    }
  }

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
