use std::{collections::HashMap, path::PathBuf};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::pdx_script_parser::parse_script;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StateBuilding {
  pub name: String,
  pub level: Option<i64>,
  pub reserves: Option<i64>,
  pub activate_production_methods: Option<Vec<String>>,
  pub condition: Option<Value>,
}

pub fn get_state_buildings(state_buildings_path: PathBuf) -> HashMap<String, Vec<StateBuilding>> {
  let mut state_buildings_map: HashMap<String, Vec<StateBuilding>> = HashMap::new();

  for entry in std::fs::read_dir(state_buildings_path).unwrap() {
    let entry = entry.unwrap().path();
    let parsed_state_buildings = parse_script(&std::fs::read_to_string(entry).unwrap());

    parsed_state_buildings[0][1].as_array().unwrap().iter().for_each(|item| {
      let (state_name, state_buildings, condition) = match item[0].as_str().unwrap() {
        "if" => {
          let block = item[1].as_array().unwrap();
          let condition = &block.iter().find(|item| item[0] == "limit").unwrap()[1];
          let state_item = block.iter().find(|item| item[0].as_str().unwrap().starts_with("s:")).unwrap();
          (state_item[0].as_str().unwrap().to_string(), state_item[1].as_array().unwrap(), Some(condition))
        },
        _ => {
          (item[0].as_str().unwrap().to_string(), item[1].as_array().unwrap(), None)
        }
      };

      state_buildings.iter().for_each(|raw_sub_state_buildings| {
        let country_name = raw_sub_state_buildings[0].as_str().unwrap().strip_prefix("region_state:").unwrap();

        let parsed_sub_state_buildings: Vec<StateBuilding> = raw_sub_state_buildings[1].as_array().unwrap().iter().map(|raw_building| {
          let building = raw_building[1].as_array().unwrap();
          let name = building.iter().find(|item| item[0] == "building").unwrap()[1].as_str().unwrap().to_string();
          let level: Option<i64> = match building.iter().find(|item| item[0] == "level") {
            Some(level) => Some(level[1].as_str().unwrap().parse().unwrap()),
            None => None
          };
          let reserves: Option<i64> = match building.iter().find(|item| item[0] == "reserves") {
            Some(level) => Some(level[1].as_str().unwrap().parse().unwrap()),
            None => None
          };
          let activate_production_methods = match building.iter().find(|item| item[0] == "activate_production_methods") {
            Some(activate_production_methods) => Some(activate_production_methods[1].as_array().unwrap().iter().map(|method| method.as_str().unwrap().trim_matches('"').to_string()).collect()),
            None => None
          };

          StateBuilding {
            name,
            level,
            reserves,
            activate_production_methods,
            condition: condition.cloned()
          }
        }).collect();

        match state_buildings_map.get_mut(&format!("{}:{}", country_name, state_name)) {
          Some(sub_state_buildings) => {
            sub_state_buildings.extend(parsed_sub_state_buildings);
          },
          None => {
            state_buildings_map.insert(format!("{}:{}", country_name, state_name), parsed_sub_state_buildings);
          }
        }
      });
    });
  };

  state_buildings_map
}
