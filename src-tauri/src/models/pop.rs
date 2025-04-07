use std::collections::HashMap;

use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "pops")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub state_id: i32,
    pub culture: String,
    pub religion: Option<String>,
    pub size: i64,
    pub pop_type: Option<String>,
}

#[derive(FromQueryResult, Debug, Serialize, Deserialize, Clone)]
pub struct SavablePop {
    pub culture: String,
    pub religion: Option<String>,
    pub size: i64,
    pub pop_type: Option<String>,
    pub state_name: String,
    pub country_tag: String,
}

pub type SavablePops = HashMap<String, HashMap<String, Vec<SavablePop>>>;

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
