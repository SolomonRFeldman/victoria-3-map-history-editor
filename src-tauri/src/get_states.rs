use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::pdx_script_parser::parse_script;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubState {
  pub provinces: Vec<String>,
  pub owner: String,
  pub coordinates: Vec<Vec<(f32, f32)>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct State {
  pub name: String,
  pub sub_states: Vec<SubState>
}

pub fn get_states(state_definition_path: PathBuf) -> Vec<State> {
  let parsed_states = parse_script(&std::fs::read_to_string(state_definition_path).unwrap());

  parsed_states[0][1].as_array().unwrap().iter().map(|state| {
    get_sub_states_from_state(state.as_array().unwrap())
  }).collect::<Vec<_>>()
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
