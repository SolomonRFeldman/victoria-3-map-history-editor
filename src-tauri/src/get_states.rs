use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::pdx_script_parser::parse_script;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubState {
  pub provinces: Vec<String>,
  pub owner: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct State {
  pub name: String,
  pub sub_states: Vec<SubState>,
  pub homelands: Vec<String>,
  pub claims: Vec<String>
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
    let sub_state_owner = &sub_state_data.iter().find(|item| item[0] == "country").unwrap().as_array().unwrap()[1].as_str().unwrap()[2..];

    let filtered_sub_state_provinces = sub_state_provinces.iter()
      .map(|province| province.as_str().unwrap().trim_matches('"').to_string()).collect::<Vec<String>>().iter()
      .filter(|province| province.len() == 7 && province.chars().nth(0).unwrap() == 'x' ).collect::<Vec<&String>>().iter()
      .map(|province| format!("x{}", province[1..].to_uppercase())).collect::<Vec<String>>();

    SubState {
      provinces: filtered_sub_state_provinces,
      owner: sub_state_owner.to_string()
    }
  }).collect::<Vec<SubState>>();

  let homelands = state[1].as_array().unwrap().iter().filter(|item| item[0] == "add_homeland").map(|homeland| {
    homeland[1].as_str().unwrap().to_string()
  }).collect::<Vec<String>>();

  let claims = state[1].as_array().unwrap().iter().filter(|item| item[0] == "add_claim").map(|claim| {
    claim[1].as_str().unwrap().to_string()
  }).collect::<Vec<String>>();

  State {
    name: state[0].as_str().unwrap().to_string(),
    sub_states,
    homelands,
    claims
  }
}
