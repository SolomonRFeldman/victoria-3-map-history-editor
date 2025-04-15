use geo::BooleanOps;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter};
use serde::Serialize;
use tauri::async_runtime::block_on;

use crate::geo_converters::{multi_poly_to_vec, vec_to_multi_poly};
use crate::models::country::Border;
use crate::models::state::Provinces;
use crate::models::{country, state};

#[derive(Serialize)]
pub struct TransferProvinceResponse {
    pub from_state: state::Model,
    pub from_country: country::Model,
    pub to_country: country::Model,
}

pub fn transfer_province(
    txn: &DatabaseTransaction,
    mut from_state: state::ActiveModel,
    mut from_country: country::ActiveModel,
    mut to_country: country::ActiveModel,
    province: String,
    province_coords: Vec<Vec<(f32, f32)>>,
) -> TransferProvinceResponse {
    let mut to_state: state::ActiveModel = match block_on(
        state::Entity::find()
            .filter(state::Column::Name.eq(from_state.name.clone().unwrap()))
            .filter(state::Column::CountryId.eq(to_country.id.clone().unwrap()))
            .one(txn),
    )
    .unwrap()
    {
        Some(state) => state.into(),
        None => state::ActiveModel {
            name: Set(from_state.name.clone().unwrap()),
            country_id: Set(to_country.id.clone().unwrap()),
            border: Set(Border(vec![])),
            provinces: Set(Provinces(vec![])),
            ..Default::default()
        },
    };

    to_state.provinces = {
        let mut combined_provinces = to_state.provinces.clone().unwrap().0;
        combined_provinces.push(province.clone());
        Set(Provinces(combined_provinces))
    };
    from_state.provinces = {
        let mut remaining_provinces = from_state.provinces.clone().unwrap().0;
        remaining_provinces.retain(|p| p != &province);
        Set(Provinces(remaining_provinces))
    };

    let province_coords = vec_to_multi_poly(province_coords);

    let to_state_coords = vec_to_multi_poly(to_state.border.unwrap().0);
    let from_state_coords = vec_to_multi_poly(from_state.border.clone().unwrap().0);
    to_state.border = Set(Border(multi_poly_to_vec(
        to_state_coords.union(&province_coords),
    )));
    from_state.border = Set(Border(multi_poly_to_vec(
        from_state_coords.difference(&province_coords),
    )));

    let from_country_coords = vec_to_multi_poly(from_country.border.unwrap().0);
    let to_country_coords = vec_to_multi_poly(to_country.border.unwrap().0);
    to_country.border = Set(Border(multi_poly_to_vec(
        to_country_coords.union(&province_coords),
    )));
    from_country.border = Set(Border(multi_poly_to_vec(
        from_country_coords.difference(&province_coords),
    )));

    let from_state = block_on(from_state.update(txn)).unwrap();
    let from_country = block_on(from_country.update(txn)).unwrap();
    let to_country = block_on(to_country.update(txn)).unwrap();
    block_on(to_state.save(txn)).unwrap();

    TransferProvinceResponse {
        from_state,
        from_country,
        to_country,
    }
}
