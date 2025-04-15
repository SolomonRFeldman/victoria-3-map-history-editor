use serde::{Deserialize, Serialize};

use crate::{
    country_setup::CountrySetup, get_state_buildings::StateBuilding, get_state_populations::Pop,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Country {
    pub name: String,
    pub color: (u8, u8, u8),
    pub states: Vec<State>,
    pub coordinates: Vec<Vec<(f32, f32)>>,
    pub setup: CountrySetup,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct State {
    pub name: String,
    pub provinces: Vec<String>,
    pub pops: Vec<Pop>,
    pub state_buildings: Vec<StateBuilding>,
}
