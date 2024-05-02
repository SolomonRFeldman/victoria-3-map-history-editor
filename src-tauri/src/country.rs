use geo::{BooleanOps, MultiPolygon};
use serde::{Deserialize, Serialize};

use crate::{
    country_definition::CountryDefinition,
    geo_converters::{multi_poly_to_vec, vec_to_multi_poly},
    get_state_buildings::StateBuilding,
    get_state_populations::Pop,
    merge_buildings::merge_state_buildings,
    merge_pops::merge_pops,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Country {
    pub name: String,
    pub color: (u8, u8, u8),
    pub states: Vec<State>,
    pub coordinates: Vec<Vec<(f32, f32)>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct State {
    pub name: String,
    pub provinces: Vec<String>,
    pub pops: Vec<Pop>,
    pub state_buildings: Vec<StateBuilding>,
}

impl Country {
    pub fn remove_state(mut self, state: &str, coords: &MultiPolygon<f32>) -> Country {
        self.states.retain(|from_state| from_state.name != state);
        self.coordinates =
            multi_poly_to_vec(vec_to_multi_poly(self.coordinates).difference(coords));

        self
    }

    pub fn new(country_definition: CountryDefinition) -> Country {
        Country {
            name: country_definition.tag,
            color: country_definition.color,
            states: vec![],
            coordinates: vec![],
        }
    }

    pub fn add_state(
        mut self,
        from_country: &Country,
        state: &str,
        to_coords: &MultiPolygon<f32>,
    ) -> Country {
        self.coordinates = multi_poly_to_vec(vec_to_multi_poly(self.coordinates).union(to_coords));
        let existing_state = self.states.iter().find(|to_state| to_state.name == state);
        match existing_state {
            Some(to_state) => {
                let mut new_provinces = to_state.provinces.clone();
                let from_state = from_country
                    .states
                    .iter()
                    .find(|from_state| from_state.name == state)
                    .unwrap();
                new_provinces.extend(from_state.provinces.clone());
                let new_pops = merge_pops(to_state.pops.clone(), from_state.pops.clone());
                let new_state_buildings = merge_state_buildings(
                    to_state.state_buildings.clone(),
                    from_state.state_buildings.clone(),
                );
                self.states.retain(|to_state| to_state.name != state);
                self.states.push(State {
                    name: state.to_string(),
                    provinces: new_provinces,
                    pops: new_pops,
                    state_buildings: new_state_buildings,
                });
            }
            None => {
                self.states.push(
                    from_country
                        .states
                        .iter()
                        .find(|from_state| from_state.name == state)
                        .unwrap()
                        .clone(),
                );
            }
        }

        self
    }

    pub fn add_province(
        mut self,
        state: &str,
        province: &str,
        province_coords: &MultiPolygon<f32>,
        pops: Vec<Pop>,
        state_buildings: Vec<StateBuilding>,
    ) -> Country {
        let existing_state = self.states.iter().find(|to_state| to_state.name == state);
        match existing_state {
            Some(to_state) => {
                let new_pops = merge_pops(to_state.pops.clone(), pops);
                let new_state_buildings =
                    merge_state_buildings(to_state.state_buildings.clone(), state_buildings);
                let mut new_provinces = to_state.provinces.clone();
                new_provinces.push(province.to_string());
                self.states.retain(|to_state| to_state.name != state);
                self.states.push(State {
                    name: state.to_string(),
                    provinces: new_provinces,
                    pops: new_pops,
                    state_buildings: new_state_buildings,
                });
            }
            None => {
                let new_state = State {
                    name: state.to_string(),
                    provinces: vec![province.to_string()],
                    pops,
                    state_buildings,
                };
                self.states.push(new_state);
            }
        };

        self.coordinates =
            multi_poly_to_vec(vec_to_multi_poly(self.coordinates).union(province_coords));

        self
    }

    pub fn remove_province(
        mut self,
        state: &str,
        province: &str,
        province_coords: &MultiPolygon<f32>,
    ) -> (Country, Vec<Pop>, Vec<StateBuilding>) {
        let existing_state = self
            .states
            .iter()
            .find(|from_state| from_state.name == state)
            .unwrap();
        let new_pops = existing_state.pops.clone();
        let new_state_buildings = existing_state.state_buildings.clone();

        let new_provinces: Vec<String> = existing_state
            .provinces
            .iter()
            .filter(|from_province| *from_province != province)
            .cloned()
            .collect();
        self.states.retain(|from_state| from_state.name != state);

        let (pops_given, state_buildings_given) = match !new_provinces.is_empty() {
            true => {
                self.states.push(State {
                    name: state.to_string(),
                    provinces: new_provinces,
                    pops: new_pops,
                    state_buildings: new_state_buildings,
                });
                (vec![], vec![])
            }
            false => (new_pops, new_state_buildings),
        };

        self.coordinates =
            multi_poly_to_vec(vec_to_multi_poly(self.coordinates).difference(province_coords));

        (self, pops_given, state_buildings_given)
    }
}
