use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "building_ownerships")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub building_id: i32,
    pub state_id: i32,
    pub owner_type: String,
    pub levels: i64,
}

#[derive(FromQueryResult, Debug, Serialize, Deserialize, Clone)]
pub struct SavableBuildingOwnership {
    pub id: i32,
    pub levels: i64,
    pub building_id: i32,
    pub owner_type: String,
    pub state_name: String,
    pub country_tag: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::building::Entity",
        from = "Column::BuildingId",
        to = "super::building::Column::Id"
    )]
    Building,
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
