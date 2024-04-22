use std::{collections::HashMap, path::PathBuf};
use serde::{Deserialize, Serialize};
use tauri::{Manager, WindowMenuEvent};

use crate::{cache_config::CacheConfig, game_folder::STATES_PATH, get_countries::Country, get_state_populations::Pop, get_states::{get_states, State}};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SubState {
  pub provinces: Vec<String>,
  pub owner: String,
  pub pops: Vec<Pop>,
}

pub fn save_as_pdx_script(event: WindowMenuEvent) {
  let start = std::time::Instant::now();
  let cache_dir = event.window().app_handle().path_resolver().app_cache_dir().unwrap();
  let cache_config: CacheConfig = serde_json::from_str(&std::fs::read_to_string(cache_dir.join("config.json")).unwrap()).unwrap();
  let game_folder = cache_config.game_folder.unwrap();
  let working_dir = cache_config.working_dir.unwrap();

  let states = get_states(game_folder.join(STATES_PATH));

  let current_countries: Vec<Country> = serde_json::from_str(&std::fs::read_to_string(cache_dir.join("countries.json")).unwrap()).unwrap();
  let mut current_state_map: HashMap<String, Vec<SubState>> = HashMap::new();
  current_countries.iter().for_each(|country| {
    country.states.iter().for_each(|state| {
      match current_state_map.get(&state.name) {
        Some(sub_states) => {
          let mut new_sub_states = sub_states.clone();
          new_sub_states.push(SubState {
            provinces: state.provinces.clone(),
            owner: country.name.clone(),
            pops: state.pops.clone()
          });
          current_state_map.insert(state.name.clone(), new_sub_states);
        },
        None => {
          current_state_map.insert(state.name.clone(), vec![SubState {
            provinces: state.provinces.clone(),
            owner: country.name.clone(),
            pops: state.pops.clone()
          }]);
        }
      }
    });
  });

  let state_pop_path = working_dir.join("common/history/pops");
  std::fs::create_dir_all(&state_pop_path).unwrap();
  write_state_pops_to_pdx_script(&current_state_map, &state_pop_path);
  overwrite_existing_pops(&state_pop_path, &game_folder.join("game/common/history/pops"));

  let states_dir = working_dir.join("common/history/states");
  std::fs::create_dir_all(&states_dir).unwrap();
  write_states_to_pdx_script(states, current_state_map, states_dir);
  println!("Saved as mod in: {:?}", start.elapsed());
}

// This might be useful someday :')
// fn compare_sub_states(state1: &[SubStateProvince], state2: &[SubState]) -> bool {
//   let state1_map: HashMap<String, Vec<String>> = state1.iter().map(|sub_state| {
//     let mut sub_state_provinces = sub_state.provinces.clone();
//     sub_state_provinces.sort();
//     (sub_state.owner.clone(), sub_state_provinces)
//   }).collect();
//   let state2_map: HashMap<String, Vec<String>> = state2.iter().map(|sub_state| {
//     let mut sub_state_provinces = sub_state.provinces.clone();
//     sub_state_provinces.sort();
//     (sub_state.owner.clone(), sub_state_provinces)
//   }).collect();

//   state1_map == state2_map
// }

fn write_states_to_pdx_script(game_states: Vec<State>, current_state_map: HashMap<String, Vec<SubState>>, path: PathBuf) {
  let mut pdx_script = String::new();

  pdx_script.push_str("STATES = {\n");
  game_states.iter().for_each(|state| {
    pdx_script.push_str(&format!("  {} = ", state.name));
    pdx_script.push_str("{\n");

    current_state_map.get(&state.name).unwrap().iter().for_each(|sub_state| {
      pdx_script.push_str("    create_state = {\n");
      pdx_script.push_str(&format!("      country = c:{}\n", sub_state.owner));

      pdx_script.push_str("      owned_provinces = { ");
      sub_state.provinces.iter().for_each(|province| {
        pdx_script.push_str(&format!("{} ", province));
      });
      pdx_script.push_str("}\n");

      pdx_script.push_str("    }\n");
    });
    state.homelands.iter().for_each(|homeland| {
      pdx_script.push_str(&format!("    add_homeland = {}\n", homeland));
    });
    state.claims.iter().for_each(|claim| {
      pdx_script.push_str(&format!("    add_claim = {}\n", claim));
    });

    pdx_script.push_str("  }\n");
  });
  pdx_script.push_str("}\n");

  std::fs::write(path.join("00_states.txt"), pdx_script).unwrap();
}

fn write_state_pops_to_pdx_script(current_state_map: &HashMap<String, Vec<SubState>>, path: &PathBuf) {
  let mut pdx_script = String::new();

  pdx_script.push_str("POPS = {\n");
  current_state_map.iter().for_each(|(state_name, sub_states)| {
    pdx_script.push_str(&format!("  {} = ", state_name));
    pdx_script.push_str("{\n");
    sub_states.iter().for_each(|sub_state| {
      pdx_script.push_str(&format!("    region_state:{} = ", sub_state.owner));
      pdx_script.push_str("{\n");
      sub_state.pops.iter().for_each(|pop| {
        pdx_script.push_str("      create_pop = {\n");
        pdx_script.push_str(&format!("        culture = {}\n", pop.culture));
        if let Some(religion) = &pop.religion {
          pdx_script.push_str(&format!("        religion = {}\n", religion));
        }
        pdx_script.push_str(&format!("        size = {}\n", pop.size));
        if let Some(pop_type) = &pop.pop_type {
          pdx_script.push_str(&format!("        pop_type = {}\n", pop_type));
        }
        pdx_script.push_str("      }\n");
      });
      pdx_script.push_str("    }\n");
    });
    
    pdx_script.push_str("  }\n");
  });
  pdx_script.push_str("}\n");
  std::fs::write(path.join("00_pops.txt"), pdx_script).unwrap();
}

fn overwrite_existing_pops(path: &PathBuf, game_pops_path: &PathBuf) {
  for entry in std::fs::read_dir(game_pops_path).unwrap() {
    let entry_name = entry.unwrap().file_name().to_str().unwrap().to_string();
    std::fs::write(path.join(entry_name), "").unwrap();
  }
}
