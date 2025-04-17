use sea_orm::{entity::prelude::*, ActiveValue::Set, FromJsonQueryResult, FromQueryResult};
use serde::{Deserialize, Serialize};

use crate::country_setup::CountrySetup;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "countries")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub tag: String,
    pub color: Color,
    pub setup: CountrySetup,
    pub border: Border,
}

#[derive(FromQueryResult, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithoutBorder {
    pub id: i32,
    pub tag: String,
    pub color: Color,
    pub setup: CountrySetup,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::state::Entity")]
    State,
}

impl Related<super::state::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::State.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Color(pub (u8, u8, u8));

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Border(pub Vec<Vec<(f32, f32)>>);

impl ActiveModel {
    pub fn new(tag: String, color: Color) -> ActiveModel {
        ActiveModel {
            tag: Set(tag),
            color: Set(color),
            setup: Set(CountrySetup::new()),
            border: Set(Border(vec![])),
            ..Default::default()
        }
    }
}
