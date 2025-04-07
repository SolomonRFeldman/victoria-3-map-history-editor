use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tauri::{async_runtime::block_on, AppHandle, Manager};

use crate::{
    cache_config::CacheConfig,
    country_setup::CountrySetup,
    game_folder::{COUNTRY_SETUP_PATH, STATES_PATH},
    get_state_buildings::StateBuilding,
    get_state_populations::Pop,
    get_states::{get_states, State},
    models::{
        building::{self, ActivateProductionMethods, SavableBuilding, SavableBuildings},
        building_ownership::{self, SavableBuildingOwnership},
        country,
        country_ownership::{self, SavableCountryOwnership},
        pop::{self, SavablePop, SavablePops},
        state::{self, SavableState},
    },
};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SubState {
    pub provinces: Vec<String>,
    pub owner: String,
    pub pops: Vec<Pop>,
    pub state_buildings: Vec<StateBuilding>,
}

pub fn save_as_pdx_script(app_handle: &AppHandle) {
    let start = std::time::Instant::now();
    let cache_dir = app_handle.path().app_cache_dir().unwrap();
    let cache_config: CacheConfig = CacheConfig::get_config(cache_dir.join("config.json"));
    let game_folder = cache_config.game_folder.unwrap();
    let working_dir = cache_config.working_dir.unwrap();
    let db = app_handle.state::<DatabaseConnection>().inner();

    let states = get_states(game_folder.join(STATES_PATH));

    let working_dir_country_setup_path = working_dir.join(COUNTRY_SETUP_PATH);
    let game_country_setup_path = game_folder.join("game").join(COUNTRY_SETUP_PATH);
    std::fs::create_dir_all(&working_dir_country_setup_path).unwrap();
    write_country_setup_to_pdx_script(db, working_dir_country_setup_path, game_country_setup_path);

    let state_pop_path = working_dir.join("common/history/pops");
    std::fs::create_dir_all(&state_pop_path).unwrap();
    write_state_pops_to_pdx_script(db, &state_pop_path);
    overwrite_existing_pops(
        &state_pop_path,
        &game_folder.join("game/common/history/pops"),
    );

    let state_buildings_path = working_dir.join("common/history/buildings");
    std::fs::create_dir_all(&state_buildings_path).unwrap();
    write_state_buildings_to_pdx_script(db, &state_buildings_path);
    overwrite_existing_buildings(
        &state_buildings_path,
        &game_folder.join("game/common/history/buildings"),
    );

    let states_dir = working_dir.join("common/history/states");
    std::fs::create_dir_all(&states_dir).unwrap();
    write_states_to_pdx_script(states, db, states_dir);
    println!("Saved as mod in: {:?}", start.elapsed());
}

fn write_country_setup_to_pdx_script(
    db: &DatabaseConnection,
    working_dir_country_setup_path: PathBuf,
    game_country_setup_path: PathBuf,
) {
    let countries = block_on(
        country::Entity::find()
            .into_model::<country::WithoutBorder>()
            .all(db),
    )
    .unwrap();

    let country_setup_map = CountrySetup::parse_map_from(game_country_setup_path.clone());
    let unprocessed_setup_map = CountrySetup::parse_map_unprocessed_values(game_country_setup_path);
    let unprocessed_working_dir_setup_map =
        CountrySetup::parse_map_unprocessed_values(working_dir_country_setup_path.clone());

    countries.iter().for_each(|country| {
        let save_path =
            working_dir_country_setup_path.join(format!("{}.txt", country.tag.to_lowercase()));
        if save_path.exists() {
            std::fs::remove_file(&save_path).unwrap();
        }

        let is_country_setup_changed = match country_setup_map.get(&country.tag) {
            Some(game_country_setup) => {
                country.setup != *game_country_setup
                    || match unprocessed_working_dir_setup_map.get(&country.tag) {
                        Some(unprocessed_working_dir_setup) => {
                            unprocessed_working_dir_setup.trim_start()
                                != unprocessed_setup_map
                                    .get(&country.tag)
                                    .unwrap()
                                    .trim_start()
                        }
                        None => false,
                    }
            }
            None => true,
        };

        if is_country_setup_changed {
            let unparsed_script = match unprocessed_working_dir_setup_map.get(&country.tag) {
                Some(script) => script,
                None => match unprocessed_setup_map.get(&country.tag) {
                    Some(script) => script,
                    None => "  ",
                },
            };
            let mut country_setup_script = String::new();
            country_setup_script.push_str("COUNTRIES = {\n");
            country_setup_script.push_str(&format!("  c:{} = ", country.tag));
            country_setup_script.push_str("{\n");
            if country.setup.base_tech.is_some() {
                country_setup_script.push_str(&format!(
                    "    effect_starting_technology_{}_tech = yes\n",
                    country.setup.base_tech.clone().unwrap()
                ));
            }
            country
                .setup
                .technologies_researched
                .iter()
                .for_each(|tech| {
                    country_setup_script
                        .push_str(&format!("    add_technology_researched = {}\n", tech));
                });
            country_setup_script.push_str(unparsed_script);
            country_setup_script.push_str("}\n");
            country_setup_script.push_str("}\n");

            std::fs::write(save_path, country_setup_script).unwrap();
        }
    });
}

fn write_states_to_pdx_script(game_states: Vec<State>, db: &DatabaseConnection, path: PathBuf) {
    let state_region_map: HashMap<String, Vec<SavableState>> = block_on(
        state::Entity::find()
            .column_as(country::Column::Tag, "country_tag")
            .join(JoinType::LeftJoin, state::Relation::Country.def())
            .into_model::<SavableState>()
            .all(db),
    )
    .unwrap()
    .into_iter()
    .fold(HashMap::new(), |mut state_region_map, state| {
        let states = state_region_map
            .entry(format!("s:{}", state.name.clone()))
            .or_default();
        states.push(state);
        state_region_map
    });

    let mut pdx_script = String::new();

    pdx_script.push_str("STATES = {\n");
    game_states.iter().for_each(|state| {
        pdx_script.push_str(&format!("  {} = ", state.name));
        pdx_script.push_str("{\n");

        state_region_map
            .get(&state.name)
            .unwrap()
            .iter()
            .for_each(|state| {
                pdx_script.push_str("    create_state = {\n");
                pdx_script.push_str(&format!("      country = c:{}\n", state.country_tag));

                pdx_script.push_str("      owned_provinces = { ");
                state.provinces.0.iter().for_each(|province| {
                    pdx_script.push_str(&format!("{} ", province));
                });
                pdx_script.push_str("}\n");

                pdx_script.push_str("    }\n");
            });
        state.homelands.iter().for_each(|homeland| {
            pdx_script.push_str(&format!("    add_homeland = {}\n", homeland));
        });
        state.claims.iter().for_each(|claim| {
            pdx_script.push_str(&format!("    add_claim = {}\n", claim));
        });

        pdx_script.push_str("  }\n");
    });
    pdx_script.push_str("}\n");

    std::fs::write(path.join("00_states.txt"), pdx_script).unwrap();
}

fn write_state_pops_to_pdx_script(db: &DatabaseConnection, path: &Path) {
    let pops = block_on(
        pop::Entity::find()
            .column_as(state::Column::Name, "state_name")
            .column_as(country::Column::Tag, "country_tag")
            .join(JoinType::LeftJoin, pop::Relation::State.def())
            .join(JoinType::LeftJoin, state::Relation::Country.def())
            .into_model::<SavablePop>()
            .all(db),
    )
    .unwrap();

    let mut pop_map: SavablePops = HashMap::new();
    pops.iter().for_each(|pop| {
        let state_name = pop.state_name.clone();
        let country_tag = pop.country_tag.clone();
        let pop_states = pop_map.entry(state_name.clone()).or_default();
        let country_pops = pop_states.entry(country_tag.clone()).or_default();
        country_pops.push(pop.clone());
    });

    let mut pdx_script = String::new();

    pdx_script.push_str("POPS = {\n");
    pop_map.iter().for_each(|(state_name, pop_states)| {
        pdx_script.push_str(&format!("  s:{} = ", state_name));
        pdx_script.push_str("{\n");
        pop_states.iter().for_each(|(country_name, pops)| {
            pdx_script.push_str(&format!("    region_state:{} = ", country_name));
            pdx_script.push_str("{\n");
            pops.iter().for_each(|pop| {
                pdx_script.push_str("      create_pop = {\n");
                pdx_script.push_str(&format!("        culture = {}\n", pop.culture));
                if let Some(religion) = &pop.religion {
                    pdx_script.push_str(&format!("        religion = {}\n", religion));
                }
                pdx_script.push_str(&format!("        size = {}\n", pop.size));
                if let Some(pop_type) = &pop.pop_type {
                    pdx_script.push_str(&format!("        pop_type = {}\n", pop_type));
                }
                pdx_script.push_str("      }\n");
            });
            pdx_script.push_str("    }\n");
        });

        pdx_script.push_str("  }\n");
    });
    pdx_script.push_str("}\n");
    std::fs::write(path.join("00_pops.txt"), pdx_script).unwrap();
}

fn overwrite_existing_pops(path: &Path, game_pops_path: &PathBuf) {
    for entry in std::fs::read_dir(game_pops_path).unwrap() {
        let entry_name = entry.unwrap().file_name().to_str().unwrap().to_string();
        std::fs::write(path.join(entry_name), "").unwrap();
    }
}

fn write_state_buildings_to_pdx_script(db: &DatabaseConnection, path: &Path) {
    let mut buildings_map: HashMap<i32, SavableBuilding> = block_on(
        building::Entity::find()
            .column_as(state::Column::Name, "state_name")
            .column_as(country::Column::Tag, "country_tag")
            .join(JoinType::LeftJoin, building::Relation::State.def())
            .join(JoinType::LeftJoin, state::Relation::Country.def())
            .into_model::<SavableBuilding>()
            .all(db),
    )
    .unwrap()
    .iter()
    .map(|building| (building.id, building.clone()))
    .collect();
    let building_ids: Vec<i32> = buildings_map.keys().cloned().collect();

    let country_ownership: Vec<SavableCountryOwnership> = block_on(
        country_ownership::Entity::find()
            .column_as(country::Column::Tag, "country_tag")
            .join(
                JoinType::LeftJoin,
                country_ownership::Relation::Country.def(),
            )
            .filter(country_ownership::Column::BuildingId.is_in(building_ids.clone()))
            .into_model::<SavableCountryOwnership>()
            .all(db),
    )
    .unwrap();
    country_ownership.iter().for_each(|ownership| {
        let building = buildings_map.get_mut(&ownership.building_id).unwrap();
        building.country_ownership.push(ownership.clone());
    });

    let building_ownership: Vec<SavableBuildingOwnership> = block_on(
        building_ownership::Entity::find()
            .column_as(state::Column::Name, "state_name")
            .column_as(country::Column::Tag, "country_tag")
            .join(
                JoinType::LeftJoin,
                building_ownership::Relation::State.def(),
            )
            .join(JoinType::LeftJoin, state::Relation::Country.def())
            .filter(building_ownership::Column::BuildingId.is_in(building_ids))
            .into_model::<SavableBuildingOwnership>()
            .all(db),
    )
    .unwrap();
    building_ownership.iter().for_each(|ownership| {
        let building = buildings_map.get_mut(&ownership.building_id).unwrap();
        building.building_ownership.push(ownership.clone());
    });

    let mut building_map: SavableBuildings = HashMap::new();
    buildings_map.values().for_each(|building| {
        let state_name = building.state_name.clone();
        let country_tag = building.country_tag.clone();
        let state_buildings = building_map.entry(state_name.clone()).or_default();
        let country_buildings: &mut Vec<SavableBuilding> =
            state_buildings.entry(country_tag.clone()).or_default();
        country_buildings.push(building.clone());
    });

    let mut pdx_script = String::new();

    pdx_script.push_str("BUILDINGS = {\n");
    building_map
        .iter()
        .for_each(|(state_name, country_buildings)| {
            let mut conditioned_buildings: Vec<SavableBuilding> = vec![];

            pdx_script.push_str(&format!("  s:{} = ", state_name));
            pdx_script.push_str("{\n");
            country_buildings
                .iter()
                .for_each(|(country_name, buildings)| {
                    pdx_script.push_str(&format!("    region_state:{} = ", country_name));
                    pdx_script.push_str("{\n");
                    for building in buildings {
                        if building.condition.is_some() {
                            conditioned_buildings.push(building.clone());
                            continue;
                        }
                        pdx_script.push_str("      create_building = {\n");
                        pdx_script.push_str(&format!("        building=\"{}\"\n", building.name));
                        if !building.country_ownership.is_empty()
                            || !building.building_ownership.is_empty()
                        {
                            pdx_script.push_str("          add_ownership = {\n");
                            building.country_ownership.iter().for_each(|country| {
                                pdx_script.push_str("            country = {\n");
                                pdx_script.push_str(&format!(
                                    "              country=\"c:{}\"\n",
                                    country.country_tag
                                ));
                                pdx_script.push_str(&format!(
                                    "              levels={}\n",
                                    country.levels
                                ));
                                pdx_script.push_str("            }\n");
                            });
                            building.building_ownership.iter().for_each(|building| {
                                pdx_script.push_str("            building = {\n");
                                pdx_script.push_str(&format!(
                                    "              type=\"{}\"\n",
                                    building.owner_type
                                ));
                                pdx_script.push_str(&format!(
                                    "              country=\"c:{}\"\n",
                                    building.country_tag
                                ));
                                pdx_script.push_str(&format!(
                                    "              levels={}\n",
                                    building.levels
                                ));
                                pdx_script.push_str(&format!(
                                    "              region=\"{}\"\n",
                                    building.state_name
                                ));
                                pdx_script.push_str("            }\n");
                            });
                            pdx_script.push_str("          }\n");
                        }
                        if let Some(level) = building.level {
                            pdx_script.push_str(&format!("        level={}\n", level));
                        }
                        if let Some(reserves) = building.reserves {
                            pdx_script.push_str(&format!("        reserves={}\n", reserves));
                        }
                        if let ActivateProductionMethods(Some(activate_production_methods)) =
                            &building.activate_production_methods
                        {
                            pdx_script.push_str("        activate_production_methods = { ");
                            activate_production_methods.iter().for_each(|method| {
                                pdx_script.push_str(&format!("\"{}\" ", method));
                            });
                            pdx_script.push_str("}\n");
                        }
                        pdx_script.push_str("      }\n");
                    }
                    pdx_script.push_str("    }\n");
                });
            pdx_script.push_str("  }\n");

            if !conditioned_buildings.is_empty() {
                pdx_script.push_str(
                    parse_building_edge_case_conditional_to_string(conditioned_buildings).as_str(),
                );
            }
        });
    pdx_script.push_str("}\n");
    std::fs::write(path.join("00_buildings.txt"), pdx_script).unwrap();
}

fn parse_building_edge_case_conditional_to_string(buildings: Vec<SavableBuilding>) -> String {
    let mut pdx_script = String::new();

    pdx_script.push_str("  if = {\n");
    pdx_script.push_str("    limit = {\n");
    let condition = buildings[0].condition.as_ref().unwrap().as_array();
    condition.unwrap().iter().for_each(|item| {
        pdx_script.push_str(&format!(
            "      {} = {}\n",
            item[0].as_str().unwrap(),
            item[1].as_str().unwrap()
        ));
    });
    pdx_script.push_str("    }\n");
    pdx_script.push_str(&format!("    s:{} = ", buildings[0].state_name));
    pdx_script.push_str("{\n");
    pdx_script.push_str(&format!(
        "      region_state:{} = ",
        buildings[0].country_tag
    ));
    pdx_script.push_str("{\n");
    buildings.iter().for_each(|building| {
        pdx_script.push_str("        create_building = {\n");
        pdx_script.push_str(&format!("          building=\"{}\"\n", building.name));
        if let Some(level) = building.level {
            pdx_script.push_str(&format!("          level={}\n", level));
        }
        if let Some(reserves) = building.reserves {
            pdx_script.push_str(&format!("          reserves={}\n", reserves));
        }
        if let ActivateProductionMethods(Some(activate_production_methods)) =
            &building.activate_production_methods
        {
            pdx_script.push_str("          activate_production_methods = { ");
            activate_production_methods.iter().for_each(|method| {
                pdx_script.push_str(&format!("\"{}\" ", method));
            });
            pdx_script.push_str("}\n");
        }
        pdx_script.push_str("        }\n");
    });
    pdx_script.push_str("      }\n");
    pdx_script.push_str("    }\n");
    pdx_script.push_str("  }\n");

    pdx_script
}

fn overwrite_existing_buildings(path: &Path, game_buildings_path: &PathBuf) {
    for entry in std::fs::read_dir(game_buildings_path).unwrap() {
        let entry_name = entry.unwrap().file_name().to_str().unwrap().to_string();
        std::fs::write(path.join(entry_name), "").unwrap();
    }
}
