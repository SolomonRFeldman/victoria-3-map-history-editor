use crate::{
    country::Border,
    merge_states::MergeStates,
    models::{building, country_ownership},
};
use geo::BooleanOps;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter, Set};
use serde::Serialize;
use tauri::async_runtime::block_on;

use crate::{
    geo_converters::{multi_poly_to_vec, vec_to_multi_poly},
    models::{country, state},
};

#[derive(Serialize)]
pub struct TransferStateResponse {
    pub from_country: country::Model,
    pub to_country: country::Model,
}

pub fn transfer_state(
    txn: &DatabaseTransaction,
    mut from_state: state::ActiveModel,
    mut from_country: country::ActiveModel,
    mut to_country: country::ActiveModel,
) -> TransferStateResponse {
    let state_coords = vec_to_multi_poly(from_state.border.clone().unwrap().0);
    let from_country_coords = vec_to_multi_poly(from_country.border.unwrap().0);
    let to_country_coords = vec_to_multi_poly(to_country.border.unwrap().0);

    from_country.border = Set(Border(multi_poly_to_vec(
        from_country_coords.difference(&state_coords),
    )));
    to_country.border = Set(Border(multi_poly_to_vec(
        to_country_coords.union(&state_coords),
    )));
    from_state.country_id = Set(to_country.id.clone().unwrap());

    let state_buildings = block_on(
        building::Entity::find()
            .filter(building::Column::StateId.eq(from_state.id.clone().unwrap()))
            .all(txn),
    )
    .unwrap();
    let mut building_country_ownerships: Vec<country_ownership::ActiveModel> = block_on(
        country_ownership::Entity::find()
            .filter(
                country_ownership::Column::BuildingId.is_in(
                    state_buildings
                        .iter()
                        .map(|building| building.id)
                        .collect::<Vec<_>>(),
                ),
            )
            .all(txn),
    )
    .unwrap()
    .into_iter()
    .map(country_ownership::ActiveModel::from)
    .collect();
    for building_country_ownership in &mut building_country_ownerships {
        building_country_ownership.country_id = Set(to_country.id.clone().unwrap());
    }

    for building_country_ownership in building_country_ownerships {
        block_on(building_country_ownership.update(txn)).unwrap();
    }
    match block_on(
        state::Entity::find()
            .filter(state::Column::Name.eq(from_state.name.clone().unwrap()))
            .filter(state::Column::CountryId.eq(to_country.id.clone().unwrap()))
            .one(txn),
    )
    .unwrap()
    {
        Some(state) => {
            MergeStates.call(txn, from_state, state.into());
        }
        None => {
            block_on(from_state.update(txn)).unwrap();
        }
    };
    let from_country = block_on(from_country.update(txn)).unwrap();
    let to_country = block_on(to_country.update(txn)).unwrap();

    TransferStateResponse {
        from_country,
        to_country,
    }
}
