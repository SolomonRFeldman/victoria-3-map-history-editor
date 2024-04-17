use serde::Serialize;

use crate::get_states::State as StateHistory;

#[derive(Serialize, Clone)]
pub struct Country {
  pub name: String,
  pub color: String,
  pub states: Vec<State>,
  pub coordinates: Vec<Vec<(f32, f32)>>,
}

#[derive(Serialize, Clone)]
pub struct State {
  pub name: String,
  pub provinces: Vec<String>,
}

pub fn get_countries(state_histories: Vec<StateHistory>) -> Vec<Country> {
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
          });
        },
        None => {
          let color = format!("x{:02x}{:02x}{:02x}", state.owner.chars().nth(0).unwrap() as u8, state.owner.chars().nth(1).unwrap() as u8, state.owner.chars().nth(2).unwrap() as u8);

          countries.push(Country {
            name: state.owner,
            color,
            states: vec![State {
              name: state_history_copy.name.clone(),
              provinces: state.provinces,
            }],
            coordinates: vec![],
          });
        },
      }
    }
  }

  countries
}
