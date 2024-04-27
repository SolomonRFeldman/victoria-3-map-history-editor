use geo::BooleanOps;
use serde::Serialize;

use crate::country::Country;
use crate::geo_converters::{multi_poly_to_vec, vec_to_multi_poly};

#[derive(Serialize)]
pub struct TransferProvinceResponse {
    to_country: Country,
    from_country: Country,
    to_state_coords: Vec<Vec<(f32, f32)>>,
    from_state_coords: Vec<Vec<(f32, f32)>>,
}

pub fn transfer_province(
    state: &str,
    province: &str,
    from_country: Country,
    to_country: Country,
    from_coords: Vec<Vec<(f32, f32)>>,
    to_coords: Vec<Vec<(f32, f32)>>,
    province_coords: Vec<Vec<(f32, f32)>>,
) -> TransferProvinceResponse {
    let start = std::time::Instant::now();

    let province_coords = vec_to_multi_poly(province_coords);
    let from_state_coords = vec_to_multi_poly(from_coords).difference(&province_coords);
    let to_state_coords = vec_to_multi_poly(to_coords).union(&province_coords);
    let (new_from_country, pops_given, new_state_buildings) =
        from_country.remove_province(state, province, &province_coords);
    let new_to_country = to_country.add_province(
        state,
        province,
        &province_coords,
        pops_given,
        new_state_buildings,
    );

    println!("Time to transfer province: {:?}", start.elapsed());
    TransferProvinceResponse {
        from_country: new_from_country,
        to_country: new_to_country,
        from_state_coords: multi_poly_to_vec(from_state_coords),
        to_state_coords: multi_poly_to_vec(to_state_coords),
    }
}
