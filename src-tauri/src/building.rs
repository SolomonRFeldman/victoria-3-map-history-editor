use std::{collections::HashMap, path::PathBuf};
use serde::Serialize;
use tauri::{Manager, Window};

use crate::{cache_config::CacheConfig, pdx_script_parser::parse_script};

// const PRODUCTION_METHODS_PATH: &str = "common/production_methods";
const PRODUCTION_METHOD_GROUPS_PATH: &str = "common/production_method_groups";
const BUILDINGS_PATH: &str = "common/buildings";

#[derive(Debug, Clone, Serialize)]
pub struct ProductionMethod {
  pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProductionMethodGroup {
  pub name: String,
  pub production_methods: Vec<ProductionMethod>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Building {
  pub building_name: String,
  pub production_method_groups: Vec<ProductionMethodGroup>,
}

impl Building {
  pub fn parse_from_game_folder(window: Window) -> Vec<Building> {
    let cache_dir = window.app_handle().path_resolver().app_cache_dir().unwrap();
    let cache_config: CacheConfig = CacheConfig::get_config(cache_dir.join("config.json"));

    Self::parse_from(cache_config.game_folder.unwrap().join("game"))
  }

  fn parse_from(path: PathBuf) -> Vec<Building> {
    let production_method_groups_map = parse_production_method_groups(path.join(PRODUCTION_METHOD_GROUPS_PATH));
    parse_building(path.join(BUILDINGS_PATH), production_method_groups_map)
  }
}

fn parse_building(path: PathBuf, pmg_map: HashMap<String, ProductionMethodGroup>) -> Vec<Building> {
  let mut bpm_vec: Vec<Building> = Vec::new();

  for entry in std::fs::read_dir(path).unwrap() {
    let entry = entry.unwrap().path();
    if entry.extension().unwrap() != "txt" { continue };
    let parsed_buildings = parse_script(&std::fs::read_to_string(entry).unwrap());

    for building in parsed_buildings.as_array().unwrap() {
      let building_name = building[0].as_str().unwrap().to_string();
      let parsed_production_method_groups: Vec<ProductionMethodGroup> = building[1].as_array().unwrap().iter()
        .find(|item| item[0] == "production_method_groups").unwrap()[1].as_array().unwrap().iter()
        .map(|group| pmg_map.get(&group.as_str().unwrap().to_string()).unwrap().clone()).collect();
      let building = Building { building_name, production_method_groups: parsed_production_method_groups };

      bpm_vec.push(building);
    }
  }

  bpm_vec
}

fn parse_production_method_groups(path: PathBuf) -> HashMap<String, ProductionMethodGroup> {
  let mut pmg_map: HashMap<String, ProductionMethodGroup> = HashMap::new();

  for entry in std::fs::read_dir(path).unwrap() {
    let entry = entry.unwrap().path();
    if entry.extension().unwrap() != "txt" { continue };
    let parsed_production_method_groups = parse_script(&std::fs::read_to_string(entry).unwrap());

    for production_method_group in parsed_production_method_groups.as_array().unwrap() {
      let production_group_name = production_method_group[0].as_str().unwrap().to_string();
      let parsed_production_methods: Vec<ProductionMethod> = production_method_group[1].as_array().unwrap().iter()
        .find(|item| item[0] == "production_methods").unwrap()[1].as_array().unwrap().iter()
        .map(|method| ProductionMethod { name: method.as_str().unwrap().to_string() }).collect();
      let production_method_group = ProductionMethodGroup { name: production_group_name, production_methods: parsed_production_methods };

      pmg_map.insert(production_method_group.name.clone(), production_method_group);
    }
  }

  pmg_map
}
