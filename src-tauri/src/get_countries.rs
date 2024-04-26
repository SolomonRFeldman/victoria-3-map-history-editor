use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::{country_definition::CountryDefinition, get_state_buildings::StateBuilding, get_state_populations::{Pop, StatePopulation}, get_states::State as StateHistory};

#[derive(Serialize, Deserialize, Clone)]
pub struct Country {
  pub name: String,
  pub color: (u8, u8, u8),
  pub states: Vec<State>,
  pub coordinates: Vec<Vec<(f32, f32)>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct State {
  pub name: String,
  pub provinces: Vec<String>,
  pub pops: Vec<Pop>,
  pub state_buildings: Vec<StateBuilding>
}

pub fn get_countries(state_histories: Vec<StateHistory>, state_pops: HashMap<String, StatePopulation>, state_buildings: HashMap<String, Vec<StateBuilding>>, country_definitions: HashMap<String, CountryDefinition>) -> Vec<Country> {
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
            state_buildings: state_buildings.get(&format!("{}:{}", state.owner, state_history_copy.name)).unwrap().clone(),
          });
        },
        None => {
          countries.push(Country {
            name: state.owner.clone(),
            color: country_definitions.get(&state.owner).unwrap().color,
            states: vec![State {
              name: state_history_copy.name.clone(),
              provinces: state.provinces,
              pops: state_pops.get(&format!("{}:{}", state.owner, state_history_copy.name)).unwrap().pops.clone(),
              state_buildings: state_buildings.get(&format!("{}:{}", state.owner, state_history_copy.name)).unwrap().clone(),
            }],
            coordinates: vec![],
          });
        },
      }
    }
  }

  countries
}
