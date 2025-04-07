use std::collections::HashMap;

use sea_orm::{entity::prelude::*, FromJsonQueryResult, FromQueryResult};
use serde::{Deserialize, Serialize};

use super::{
    building_ownership::SavableBuildingOwnership, country_ownership::SavableCountryOwnership,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "buildings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub state_id: i32,
    pub name: String,
    pub level: Option<i64>,
    pub reserves: Option<i64>,
    pub activate_production_methods: ActivateProductionMethods,
    pub condition: Option<Json>,
}

#[derive(FromQueryResult, Debug, Serialize, Deserialize, Clone)]
pub struct SavableBuilding {
    pub id: i32,
    pub name: String,
    pub level: Option<i64>,
    pub reserves: Option<i64>,
    pub activate_production_methods: ActivateProductionMethods,
    pub condition: Option<Json>,
    pub state_name: String,
    pub country_tag: String,
    #[sea_orm(skip)]
    pub country_ownership: Vec<SavableCountryOwnership>,
    #[sea_orm(skip)]
    pub building_ownership: Vec<SavableBuildingOwnership>,
}

pub type SavableBuildings = HashMap<String, HashMap<String, Vec<SavableBuilding>>>;

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::state::Entity",
        from = "Column::StateId",
        to = "super::state::Column::Id"
    )]
    State,
}

impl Related<super::state::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::State.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ActivateProductionMethods(pub Option<Vec<String>>);
