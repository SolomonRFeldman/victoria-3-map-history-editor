use geo::{BooleanOps, MultiPolygon};
use serde::Serialize;

use crate::get_countries::{Country, State};
use crate::geo_converters::{multi_poly_to_vec, vec_to_multi_poly};

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
  let new_to_country = add_province_to_country(to_country.clone(), state, province, &province_coords);
  let new_from_country = remove_province_from_country(from_country.clone(), state, province, &province_coords);

  println!("Time to transfer province: {:?}", start.elapsed());
  TransferProvinceResponse {
    from_country: new_from_country,
    to_country: new_to_country,
    from_state_coords: multi_poly_to_vec(from_state_coords),
    to_state_coords: multi_poly_to_vec(to_state_coords),
  }
}

fn add_province_to_country(mut country: Country, state: &str, province: &str, province_coords: &MultiPolygon<f32>) -> Country {
  let existing_state = country.states.iter().find(|to_state| to_state.name == state);
  match existing_state {
    Some(to_state) => {
      let mut new_provinces = to_state.provinces.clone();
      new_provinces.push(province.to_string());
      country.states = country.states.iter().filter(|to_state| to_state.name != state).cloned().collect();
      country.states.push(State {
        name: state.to_string(),
        provinces: new_provinces,
      });
    },
    None => {
      let new_state = State {
        name: state.to_string(),
        provinces: vec![province.to_string()],
      };
      country.states.push(new_state);
    },
  };

  country.coordinates = multi_poly_to_vec(vec_to_multi_poly(country.coordinates).union(&province_coords));
  country
}

fn remove_province_from_country(mut country: Country, state: &str, province: &str, province_coords: &MultiPolygon<f32>) -> Country {
  let existing_state = country.states.iter().find(|from_state| from_state.name == state).unwrap();
  
  let new_provinces: Vec<String> = existing_state.provinces.iter().filter(|from_province| **from_province != province.to_string()).cloned().collect();
  country.states = country.states.iter().filter(|from_state| from_state.name != state).cloned().collect();
  if new_provinces.len() > 0 {
    country.states.push(State {
      name: state.to_string(),
      provinces: new_provinces,
    });
  }

  country.coordinates = multi_poly_to_vec(vec_to_multi_poly(country.coordinates).difference(&province_coords));
  country
}
