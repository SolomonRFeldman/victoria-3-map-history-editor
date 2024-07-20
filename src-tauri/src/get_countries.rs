use std::collections::HashMap;

use crate::{
    country::{Country, State},
    country_definition::CountryDefinition,
    country_setup::CountrySetup,
    get_state_buildings::StateBuilding,
    get_state_populations::StatePopulation,
    get_states::State as StateHistory,
};

pub fn get_countries(
    state_histories: Vec<StateHistory>,
    state_pops: HashMap<String, StatePopulation>,
    state_buildings: HashMap<String, Vec<StateBuilding>>,
    country_definitions: HashMap<String, CountryDefinition>,
    country_setups: HashMap<String, CountrySetup>,
) -> Vec<Country> {
    let mut countries: Vec<Country> = vec![];

    for state_history in state_histories {
        let state_history_copy = state_history.clone();

        for state in state_history.sub_states {
            let country = countries
                .iter_mut()
                .find(|country| country.name == state.owner);
            let pops = state_pops
                .get(&format!("{}:{}", state.owner, state_history_copy.name))
                .unwrap()
                .pops
                .clone();
            let state_buildings = match state_buildings
                .get(&format!("{}:{}", state.owner, state_history_copy.name))
            {
                Some(state_buildings) => state_buildings.clone(),
                None => vec![],
            }
            .to_vec();

            match country {
                Some(country) => {
                    country.states.push(State {
                        name: state_history_copy.name.clone(),
                        provinces: state.provinces,
                        pops,
                        state_buildings,
                    });
                }
                None => {
                    countries.push(Country {
                        name: state.owner.clone(),
                        color: country_definitions.get(&state.owner).unwrap().color,
                        states: vec![State {
                            name: state_history_copy.name.clone(),
                            provinces: state.provinces,
                            pops,
                            state_buildings,
                        }],
                        coordinates: vec![],
                        setup: country_setups.get(&state.owner).unwrap().clone(),
                    });
                }
            }
        }
    }

    countries
}
