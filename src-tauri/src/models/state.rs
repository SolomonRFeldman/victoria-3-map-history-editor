use sea_orm::{entity::prelude::*, FromJsonQueryResult, FromQueryResult};
use serde::{Deserialize, Serialize};

use super::country::Border;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "states")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub country_id: i32,
    pub name: String,
    pub provinces: Provinces,
    pub border: Border,
}

#[derive(FromQueryResult)]
pub struct WithoutBorder {
    pub id: i32,
    pub country_id: i32,
    pub name: String,
}

#[derive(FromQueryResult, Debug, Serialize, Deserialize, Clone)]
pub struct SavableState {
    pub id: i32,
    pub name: String,
    pub country_tag: String,
    pub provinces: Provinces,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Provinces(pub Vec<String>);
