use geo::{BooleanOps, LineString, MultiPolygon, Polygon};
use serde::Serialize;

use crate::get_countries::{Country, State};

#[derive(Serialize)]
pub struct TransferStateResponse {
  to_country: Country,
  from_country: Country,
  state_coords: Vec<Vec<(f32, f32)>>,
}

pub fn transfer_state (state: &str, from_country: Country, to_country: Country, from_coords: Vec<Vec<(f32, f32)>>, to_coords: Vec<Vec<(f32, f32)>>) -> TransferStateResponse {
  let start = std::time::Instant::now();
  let from_multi_poly: MultiPolygon<f32> = vec_to_multi_poly(from_coords);
  let to_multi_poly: MultiPolygon<f32> = vec_to_multi_poly(to_coords);
  let union = from_multi_poly.union(&to_multi_poly);
  let new_state_coords = multi_poly_to_vec(union.clone());

  let new_from_country_coords = multi_poly_to_vec(vec_to_multi_poly(from_country.coordinates.clone()).difference(&union));
  let mut new_from_country = from_country.clone();
  new_from_country.states = from_country.states.iter().filter(|from_state| from_state.name != state).cloned().collect();
  new_from_country.coordinates = new_from_country_coords;
  

  let new_to_country_coords = multi_poly_to_vec(vec_to_multi_poly(to_country.coordinates.clone()).union(&union));
  let mut new_to_country = to_country.clone();
  let existing_state = new_to_country.states.iter().find(|to_state| to_state.name == state);
  new_to_country.coordinates = new_to_country_coords;
  match existing_state {
    Some(to_state) => {
      let mut new_provinces = to_state.provinces.clone();
      new_provinces.extend(from_country.states.iter().find(|from_state| from_state.name == state).unwrap().provinces.clone());
      new_to_country.states = new_to_country.states.iter().filter(|to_state| to_state.name != state).cloned().collect();
      new_to_country.states.push(State {
        name: state.to_string(),
        provinces: new_provinces,
      });
    },
    None => {
      new_to_country.states.push(from_country.states.iter().find(|from_state| from_state.name == state).unwrap().clone());
    },
  }
  println!("Time to transfer state: {:?}", start.elapsed());
  TransferStateResponse {
    to_country: new_to_country,
    from_country: new_from_country,
    state_coords: new_state_coords,
  }
}


fn multi_poly_to_vec(multi_poly: MultiPolygon<f32>) -> Vec<Vec<(f32, f32)>> {
  let mut exterior_coords = multi_poly.clone().into_iter().map(|poly| { line_string_to_vec(poly.exterior().clone()) }).collect::<Vec<Vec<(f32, f32)>>>();
  let interior_coords = multi_poly.clone().into_iter().map(|poly| { poly.interiors().iter().map(|line_string| { line_string_to_vec(line_string.clone()) }).collect::<Vec<Vec<(f32, f32)>>>() }).collect::<Vec<Vec<Vec<(f32, f32)>>>>();
  exterior_coords.extend(interior_coords.into_iter().flatten());
  exterior_coords
}

fn line_string_to_vec(line_string: LineString<f32>) -> Vec<(f32, f32)> {
  line_string.into_iter().map(|point| { (point.x, point.y) }).collect()
}

fn vec_to_multi_poly(coords: Vec<Vec<(f32, f32)>>) -> MultiPolygon<f32> {
  coords.into_iter().map(|coords| { Polygon::new(LineString::from(coords), vec![]) }).collect()
}
