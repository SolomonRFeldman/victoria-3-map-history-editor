use geo::{BooleanOps, MultiPolygon};
use serde::Serialize;

use crate::get_countries::{Country, State};
use crate::geo_converters::{multi_poly_to_vec, vec_to_multi_poly};
use crate::merge_buildings::merge_state_buildings;
use crate::merge_pops::merge_pops;

#[derive(Serialize)]
pub struct TransferStateResponse {
  to_country: Country,
  from_country: Country,
  state_coords: Vec<Vec<(f32, f32)>>,
}

pub fn transfer_state (state: &str, from_country: Country, to_country: Country, from_coords: Vec<Vec<(f32, f32)>>, to_coords: Vec<Vec<(f32, f32)>>) -> TransferStateResponse {
  let start = std::time::Instant::now();

  let from_coords = vec_to_multi_poly(from_coords);
  let union = from_coords.union(&vec_to_multi_poly(to_coords));

  let new_to_country = add_state_to_country(to_country.clone(), &from_country, state, &from_coords);
  let new_from_country = remove_state_from_country(from_country.clone(), state, &from_coords);
  let new_state_coords = multi_poly_to_vec(union);

  println!("Time to transfer state: {:?}", start.elapsed());

  TransferStateResponse {
    from_country: new_from_country,
    to_country: new_to_country,
    state_coords: new_state_coords,
  }
}

fn add_state_to_country(mut country: Country, from_country: &Country, state: &str, to_coords: &MultiPolygon<f32>) -> Country {
  country.coordinates = multi_poly_to_vec(vec_to_multi_poly(country.coordinates).union(&to_coords));
  let existing_state = country.states.iter().find(|to_state| to_state.name == state);
  match existing_state {
    Some(to_state) => {
      let mut new_provinces = to_state.provinces.clone();
      let from_state = from_country.states.iter().find(|from_state| from_state.name == state).unwrap();
      new_provinces.extend(from_state.provinces.clone());
      let new_pops = merge_pops(to_state.pops.clone(), from_state.pops.clone());
      let new_state_buildings = merge_state_buildings(to_state.state_buildings.clone(), from_state.state_buildings.clone());
      country.states = country.states.iter().filter(|to_state| to_state.name != state).cloned().collect();
      country.states.push(State {
        name: state.to_string(),
        provinces: new_provinces,
        pops: new_pops,
        state_buildings: new_state_buildings
      });
    },
    None => {
      country.states.push(from_country.states.iter().find(|from_state| from_state.name == state).unwrap().clone());
    },
  }

  country
}

fn remove_state_from_country(mut country: Country, state: &str, coords: &MultiPolygon<f32>) -> Country {
  country.states = country.states.iter().filter(|from_state| from_state.name != state).cloned().collect();
  country.coordinates = multi_poly_to_vec(vec_to_multi_poly(country.coordinates).difference(&coords));

  country
}
