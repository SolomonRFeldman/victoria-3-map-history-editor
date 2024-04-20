use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::{get_state_populations::{Pop, StatePopulation}, get_states::State as StateHistory};

#[derive(Serialize, Deserialize, Clone)]
pub struct Country {
  pub name: String,
  pub color: String,
  pub states: Vec<State>,
  pub coordinates: Vec<Vec<(f32, f32)>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct State {
  pub name: String,
  pub provinces: Vec<String>,
  pub pops: Vec<Pop>,
}

pub fn get_countries(state_histories: Vec<StateHistory>, state_pops: HashMap<String, StatePopulation>) -> Vec<Country> {
  let mut countries: Vec<Country> = vec![];

  for state_history in state_histories {
    let state_history_copy = state_history.clone();

    for state in state_history.sub_states {
      let country = countries.iter_mut().find(|country| country.name == state.owner);

      match country {
        Some(country) => {
          country.states.push(State {
            name: state_history_copy.name.clone(),
            provinces: state.provinces,
            pops: state_pops.get(&format!("{}:{}", state.owner, state_history_copy.name)).unwrap().pops.clone(),
          });
        },
        None => {
          let color = format!("x{:02x}{:02x}{:02x}", state.owner.chars().nth(0).unwrap() as u8, state.owner.chars().nth(1).unwrap() as u8, state.owner.chars().nth(2).unwrap() as u8);
          println!("{:?}", state_history_copy.name);
          println!("{:?}", state.owner);

          countries.push(Country {
            name: state.owner.clone(),
            color,
            states: vec![State {
              name: state_history_copy.name.clone(),
              provinces: state.provinces,
              pops: state_pops.get(&format!("{}:{}", state.owner, state_history_copy.name)).unwrap().pops.clone(),
            }],
            coordinates: vec![],
          });
        },
      }
    }
  }

  countries
}
