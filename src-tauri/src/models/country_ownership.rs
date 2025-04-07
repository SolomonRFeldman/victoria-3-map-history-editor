use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "country_ownerships")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub building_id: i32,
    pub country_id: i32,
    pub levels: i64,
}

#[derive(FromQueryResult, Debug, Serialize, Deserialize, Clone)]
pub struct SavableCountryOwnership {
    pub id: i32,
    pub levels: i64,
    pub building_id: i32,
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
        belongs_to = "super::country::Entity",
        from = "Column::CountryId",
        to = "super::country::Column::Id"
    )]
    Country,
}

impl Related<super::country::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Country.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
