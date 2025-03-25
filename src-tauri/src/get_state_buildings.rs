use jomini::{
    text::{ScalarReader, ValueReader},
    JominiDeserialize, TextTape, Windows1252Encoding,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fs::read, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StateBuilding {
    pub name: String,
    pub level: Option<i64>,
    pub reserves: Option<i64>,
    pub activate_production_methods: Option<Vec<String>>,
    pub condition: Option<Value>,
    pub ownership: Option<Ownership>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RawStateBuilding {
    building: String,
    level: Option<i64>,
    reserves: Option<i64>,
    activate_production_methods: Option<Vec<String>>,
    add_ownership: Option<RawOwnership>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ownership {
    pub countries: Vec<CountryOwnership>,
    pub buildings: Vec<BuildingOwnership>,
}

#[derive(JominiDeserialize, Debug, Serialize, Clone)]
struct RawOwnership {
    #[jomini(alias = "country", duplicated)]
    countries: Vec<CountryOwnership>,
    #[jomini(alias = "building", duplicated)]
    buildings: Vec<BuildingOwnership>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CountryOwnership {
    pub country: String,
    pub levels: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildingOwnership {
    #[serde(rename = "type")]
    pub type_: String,
    pub country: String,
    pub levels: i64,
    pub region: String,
}

fn parse_state_building(
    key: ScalarReader<Windows1252Encoding>,
    value: ValueReader<Windows1252Encoding>,
) -> HashMap<String, Vec<RawStateBuilding>> {
    let state_name = key.read_str();

    value
        .read_object()
        .unwrap()
        .fields()
        .map(|(key, _op, value)| {
            let binding = key.read_str();
            let country_name = binding.strip_prefix("region_state:").unwrap();
            let state_building_name = format!("{}:{}", country_name, state_name);

            let state_buildings = value
                .read_object()
                .unwrap()
                .fields()
                .map(|(_key, _op, value)| value.read_object().unwrap().deserialize().unwrap())
                .collect();

            (state_building_name, state_buildings)
        })
        .collect()
}

pub fn get_state_buildings(state_buildings_path: PathBuf) -> HashMap<String, Vec<StateBuilding>> {
    let mut state_buildings_map: HashMap<String, Vec<StateBuilding>> = HashMap::new();

    for entry in std::fs::read_dir(state_buildings_path).unwrap() {
        let entry = entry.unwrap().path();
        let string_entry = read(entry).unwrap();
        let tape = TextTape::from_slice(&string_entry).unwrap();
        let reader = tape.windows1252_reader();

        for (_key, _op, value) in reader.fields() {
            value
                .read_object()
                .unwrap()
                .fields()
                .for_each(|(key, _op, value)| {
                    let string_key = key.read_str();
                    if string_key == "if" {
                        let mut object_value = value.read_object().unwrap().fields();

                        let (_key, _op, condition_reader) = object_value
                            .find(|(key, _op, _value)| key.read_str() == "limit")
                            .unwrap();
                        let condition: Vec<Vec<String>> = condition_reader
                            .read_object()
                            .unwrap()
                            .fields()
                            .map(|(key, _op, value)| {
                                vec![key.read_string(), value.read_string().unwrap().to_string()]
                            })
                            .collect();

                        object_value.for_each(|(key, _op, value)| {
                            if key.read_str().starts_with("s:") {
                                parse_state_building(key, value).iter().for_each(
                                    |(state_name, raw_state_building)| {
                                        let state_buildings: Vec<StateBuilding> =
                                            raw_state_building
                                                .iter()
                                                .map(|raw_state_building| StateBuilding {
                                                    name: raw_state_building.building.clone(),
                                                    level: raw_state_building.level,
                                                    reserves: raw_state_building.reserves,
                                                    activate_production_methods: raw_state_building
                                                        .activate_production_methods
                                                        .clone(),
                                                    condition: Some(condition.clone().into()),
                                                    ownership: raw_state_building
                                                        .add_ownership
                                                        .as_ref()
                                                        .map(|add_ownership| Ownership {
                                                            countries: add_ownership
                                                                .countries
                                                                .clone(),
                                                            buildings: add_ownership
                                                                .buildings
                                                                .clone(),
                                                        }),
                                                })
                                                .collect();

                                        match state_buildings_map.get_mut(state_name) {
                                            Some(sub_state_buildings) => {
                                                sub_state_buildings.extend(state_buildings.clone());
                                            }
                                            None => {
                                                state_buildings_map.insert(
                                                    state_name.clone(),
                                                    state_buildings.clone(),
                                                );
                                            }
                                        }
                                    },
                                );
                            }
                        });
                    } else {
                        parse_state_building(key, value).iter().for_each(
                            |(state_name, raw_state_building)| {
                                let state_buildings: Vec<StateBuilding> = raw_state_building
                                    .iter()
                                    .map(|raw_state_building| StateBuilding {
                                        name: raw_state_building.building.clone(),
                                        level: raw_state_building.level,
                                        reserves: raw_state_building.reserves,
                                        activate_production_methods: raw_state_building
                                            .activate_production_methods
                                            .clone(),
                                        condition: None,
                                        ownership: raw_state_building.add_ownership.as_ref().map(
                                            |add_ownership| Ownership {
                                                countries: add_ownership.countries.clone(),
                                                buildings: add_ownership.buildings.clone(),
                                            },
                                        ),
                                    })
                                    .collect();

                                match state_buildings_map.get_mut(state_name) {
                                    Some(sub_state_buildings) => {
                                        sub_state_buildings.extend(state_buildings.clone());
                                    }
                                    None => {
                                        state_buildings_map
                                            .insert(state_name.clone(), state_buildings.clone());
                                    }
                                }
                            },
                        );
                    }
                });
        }
    }

    state_buildings_map
}
