use geo::BooleanOps;
use serde::Serialize;

use crate::geo_converters::{multi_poly_to_vec, vec_to_multi_poly};
use crate::legacy_country::Country;

#[derive(Serialize)]
pub struct TransferStateResponse {
    pub to_country: Country,
    pub from_country: Country,
    pub state_coords: Vec<Vec<(f32, f32)>>,
}

pub fn transfer_state(
    state: &str,
    from_country: Country,
    to_country: Country,
    from_coords: Vec<Vec<(f32, f32)>>,
    to_coords: Vec<Vec<(f32, f32)>>,
) -> TransferStateResponse {
    let start = std::time::Instant::now();

    let from_coords = vec_to_multi_poly(from_coords);
    let union = from_coords.union(&vec_to_multi_poly(to_coords));

    let new_to_country = to_country.add_state(&from_country, state, &from_coords);
    let new_from_country = from_country.remove_state(state, &from_coords);
    let new_state_coords = multi_poly_to_vec(union);

    println!("Time to transfer state: {:?}", start.elapsed());

    TransferStateResponse {
        from_country: new_from_country,
        to_country: new_to_country,
        state_coords: new_state_coords,
    }
}
