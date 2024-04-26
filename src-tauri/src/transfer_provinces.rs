use std::vec;
use geo::{BooleanOps, MultiPolygon};
use serde::Serialize;

use crate::get_countries::{Country, State};
use crate::geo_converters::{multi_poly_to_vec, vec_to_multi_poly};
use crate::get_state_buildings::StateBuilding;
use crate::get_state_populations::Pop;
use crate::merge_buildings::merge_state_buildings;
use crate::merge_pops::merge_pops;

#[derive(Serialize)]
pub struct TransferProvinceResponse {
  to_country: Country,
  from_country: Country,
  to_state_coords: Vec<Vec<(f32, f32)>>,
  from_state_coords: Vec<Vec<(f32, f32)>>,
}

pub fn transfer_province (state: &str, province: &str, from_country: Country, to_country: Country, from_coords: Vec<Vec<(f32, f32)>>, to_coords: Vec<Vec<(f32, f32)>>, province_coords: Vec<Vec<(f32, f32)>>) -> TransferProvinceResponse {
  let start = std::time::Instant::now();

  let province_coords = vec_to_multi_poly(province_coords);
  let from_state_coords = vec_to_multi_poly(from_coords).difference(&province_coords);
  let to_state_coords = vec_to_multi_poly(to_coords).union(&province_coords);
  let (new_from_country, pops_given, new_state_buildings) = remove_province_from_country(from_country.clone(), state, province, &province_coords);
  let new_to_country = add_province_to_country(to_country.clone(), state, province, &province_coords, pops_given, new_state_buildings);

  println!("Time to transfer province: {:?}", start.elapsed());
  TransferProvinceResponse {
    from_country: new_from_country,
    to_country: new_to_country,
    from_state_coords: multi_poly_to_vec(from_state_coords),
    to_state_coords: multi_poly_to_vec(to_state_coords),
  }
}

fn add_province_to_country(mut country: Country, state: &str, province: &str, province_coords: &MultiPolygon<f32>, pops_given: Vec<Pop>, new_state_buildings: Vec<StateBuilding>) -> Country {
  let existing_state = country.states.iter().find(|to_state| to_state.name == state);
  match existing_state {
    Some(to_state) => {
      let new_pops = merge_pops(to_state.pops.clone(), pops_given);
      let new_state_buildings = merge_state_buildings(to_state.state_buildings.clone(), new_state_buildings);
      let mut new_provinces = to_state.provinces.clone();
      new_provinces.push(province.to_string());
      country.states.retain(|to_state| to_state.name != state);
      country.states.push(State {
        name: state.to_string(),
        provinces: new_provinces,
        pops: new_pops,
        state_buildings: new_state_buildings
      });
    },
    None => {
      let new_state = State {
        name: state.to_string(),
        provinces: vec![province.to_string()],
        pops: vec![],
        state_buildings: vec![]
      };
      country.states.push(new_state);
    },
  };

  country.coordinates = multi_poly_to_vec(vec_to_multi_poly(country.coordinates).union(province_coords));
  country
}

fn remove_province_from_country(mut country: Country, state: &str, province: &str, province_coords: &MultiPolygon<f32>) -> (Country, Vec<Pop>, Vec<StateBuilding>) {
  let existing_state = country.states.iter().find(|from_state| from_state.name == state).unwrap();
  let new_pops = existing_state.pops.clone();
  let new_state_buildings = existing_state.state_buildings.clone();
  
  let new_provinces: Vec<String> = existing_state.provinces.iter().filter(|from_province| *from_province != province).cloned().collect();
  country.states.retain(|from_state| from_state.name != state);

  let (pops_given, state_buildings_given) = match !new_provinces.is_empty() {
    true => {
      country.states.push(State {
        name: state.to_string(),
        provinces: new_provinces,
        pops: new_pops,
        state_buildings: new_state_buildings
      });
      (vec![], vec![])
    },
    false => (new_pops, new_state_buildings)
  };

  country.coordinates = multi_poly_to_vec(vec_to_multi_poly(country.coordinates).difference(province_coords));
  (country, pops_given, state_buildings_given)
}
