use jomini::text::de::from_utf8_reader;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use tauri::{Manager, Window};

use crate::cache_config::CacheConfig;

// const PRODUCTION_METHODS_PATH: &str = "common/production_methods";
const PRODUCTION_METHOD_GROUPS_PATH: &str = "common/production_method_groups";
const BUILDINGS_PATH: &str = "common/buildings";

#[derive(Debug, Clone, Serialize)]
pub struct ProductionMethod {
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProductionMethodGroup {
    pub name: String,
    pub production_methods: Vec<ProductionMethod>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Building {
    pub name: String,
    pub production_method_groups: Vec<ProductionMethodGroup>,
    pub buildable: bool,
    pub unique: bool,
}

impl Building {
    pub fn parse_from_game_folder(window: Window) -> Vec<Building> {
        let cache_dir = window.app_handle().path_resolver().app_cache_dir().unwrap();
        let cache_config: CacheConfig = CacheConfig::get_config(cache_dir.join("config.json"));

        Self::parse_from(cache_config.game_folder.unwrap().join("game"))
    }

    fn parse_from(path: PathBuf) -> Vec<Building> {
        let production_method_groups_map =
            parse_production_method_groups(path.join(PRODUCTION_METHOD_GROUPS_PATH));
        parse_buildings(path.join(BUILDINGS_PATH), production_method_groups_map)
    }
}

fn default_as_true() -> bool {
    true
}
#[derive(Deserialize)]
struct RawBuilding {
    production_method_groups: Vec<String>,
    #[serde(default = "default_as_true")]
    buildable: bool,
    #[serde(default)]
    unique: bool,
}

pub fn parse_buildings(
    buildings_path: PathBuf,
    pmg_map: HashMap<String, ProductionMethodGroup>,
) -> Vec<Building> {
    let mut buildings: Vec<Building> = Vec::new();

    for entry in std::fs::read_dir(buildings_path).unwrap() {
        let entry = entry.unwrap().path();
        if entry.extension().unwrap() != "txt" {
            continue;
        };
        let parsed_buildings: HashMap<String, RawBuilding> =
            from_utf8_reader(&*std::fs::read(entry).unwrap()).unwrap();

        for (name, raw_building) in parsed_buildings {
            let RawBuilding {
                production_method_groups,
                buildable,
                unique,
            } = raw_building;
            let production_method_groups: Vec<ProductionMethodGroup> = production_method_groups
                .iter()
                .map(|group| pmg_map.get(group).unwrap().clone())
                .collect();
            let building = Building {
                name: name.clone(),
                production_method_groups,
                buildable,
                unique,
            };

            buildings.push(building);
        }
    }

    buildings
}

#[derive(Deserialize)]
struct RawProductionMethodGroup {
    production_methods: Vec<String>,
}

fn parse_production_method_groups(path: PathBuf) -> HashMap<String, ProductionMethodGroup> {
    let mut pmg_map: HashMap<String, ProductionMethodGroup> = HashMap::new();

    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap().path();
        if entry.extension().unwrap() != "txt" {
            continue;
        };
        let parsed_production_method_groups: HashMap<String, RawProductionMethodGroup> =
            from_utf8_reader(&*std::fs::read(entry).unwrap()).unwrap();

        for (group_name, raw_group) in parsed_production_method_groups {
            let production_methods: Vec<ProductionMethod> = raw_group
                .production_methods
                .iter()
                .map(|method| ProductionMethod {
                    name: method.clone(),
                })
                .collect();
            let production_method_group = ProductionMethodGroup {
                name: group_name.clone(),
                production_methods,
            };

            pmg_map.insert(group_name, production_method_group);
        }
    }

    pmg_map
}
