use geo::BooleanOps;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter,
};
use tauri::async_runtime::block_on;

use crate::{
    geo_converters::{multi_poly_to_vec, vec_to_multi_poly},
    models::{
        building, building_ownership,
        country::Border,
        pop,
        state::{self, Provinces},
    },
};

pub struct MergeStates;

impl MergeStates {
    pub fn call(
        &self,
        txn: &DatabaseTransaction,
        from_state: state::ActiveModel,
        mut to_state: state::ActiveModel,
    ) -> state::ActiveModel {
        let state_coords = vec_to_multi_poly(from_state.border.clone().unwrap().0);
        let to_state_coords = vec_to_multi_poly(to_state.border.unwrap().0);

        to_state.border = Set(Border(multi_poly_to_vec(
            to_state_coords.union(&state_coords),
        )));
        to_state.provinces = {
            let mut combined_provinces = to_state.provinces.clone().unwrap().0;
            combined_provinces.extend(from_state.provinces.clone().unwrap().0);
            Set(Provinces(combined_provinces))
        };

        let state_ownerships: Vec<building_ownership::ActiveModel> = block_on(
            building_ownership::Entity::find()
                .filter(building_ownership::Column::StateId.eq(from_state.id.clone().unwrap()))
                .all(txn),
        )
        .unwrap()
        .into_iter()
        .map(building_ownership::ActiveModel::from)
        .collect();
        let state_buildings: Vec<building::ActiveModel> = block_on(
            building::Entity::find()
                .filter(building::Column::StateId.eq(from_state.id.clone().unwrap()))
                .all(txn),
        )
        .unwrap()
        .into_iter()
        .map(building::ActiveModel::from)
        .collect();
        let state_pops: Vec<pop::ActiveModel> = block_on(
            pop::Entity::find()
                .filter(pop::Column::StateId.eq(from_state.id.clone().unwrap()))
                .all(txn),
        )
        .unwrap()
        .into_iter()
        .map(pop::ActiveModel::from)
        .collect();

        let to_state = block_on(to_state.save(txn)).unwrap();
        for mut ownership in state_ownerships {
            ownership.state_id = Set(to_state.id.clone().unwrap());
            block_on(ownership.update(txn)).unwrap();
        }
        for mut building in state_buildings {
            building.state_id = Set(to_state.id.clone().unwrap());
            block_on(building.update(txn)).unwrap();
        }
        for mut pop in state_pops {
            pop.state_id = Set(to_state.id.clone().unwrap());
            block_on(pop.update(txn)).unwrap();
        }
        block_on(from_state.delete(txn)).unwrap();

        to_state
    }
}
