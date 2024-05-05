use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tauri::{Manager, WindowMenuEvent};

use crate::{
    cache_config::CacheConfig,
    country::Country,
    country_setup::CountrySetup,
    game_folder::{COUNTRY_SETUP_PATH, STATES_PATH},
    get_state_buildings::StateBuilding,
    get_state_populations::Pop,
    get_states::{get_states, State},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SubState {
    pub provinces: Vec<String>,
    pub owner: String,
    pub pops: Vec<Pop>,
    pub state_buildings: Vec<StateBuilding>,
}

pub fn save_as_pdx_script(event: WindowMenuEvent) {
    let start = std::time::Instant::now();
    let cache_dir = event
        .window()
        .app_handle()
        .path_resolver()
        .app_cache_dir()
        .unwrap();
    let cache_config: CacheConfig = CacheConfig::get_config(cache_dir.join("config.json"));
    let game_folder = cache_config.game_folder.unwrap();
    let working_dir = cache_config.working_dir.unwrap();

    let states = get_states(game_folder.join(STATES_PATH));

    let current_countries: Vec<Country> =
        serde_json::from_str(&std::fs::read_to_string(cache_dir.join("countries.json")).unwrap())
            .unwrap();
    let mut current_state_map: HashMap<String, Vec<SubState>> = HashMap::new();
    current_countries.iter().for_each(|country| {
        country
            .states
            .iter()
            .for_each(|state| match current_state_map.get(&state.name) {
                Some(sub_states) => {
                    let mut new_sub_states = sub_states.clone();
                    new_sub_states.push(SubState {
                        provinces: state.provinces.clone(),
                        owner: country.name.clone(),
                        pops: state.pops.clone(),
                        state_buildings: state.state_buildings.clone(),
                    });
                    current_state_map.insert(state.name.clone(), new_sub_states);
                }
                None => {
                    current_state_map.insert(
                        state.name.clone(),
                        vec![SubState {
                            provinces: state.provinces.clone(),
                            owner: country.name.clone(),
                            pops: state.pops.clone(),
                            state_buildings: state.state_buildings.clone(),
                        }],
                    );
                }
            });
    });

    let working_dir_country_setup_path = working_dir.join(COUNTRY_SETUP_PATH);
    let game_country_setup_path = game_folder.join("game").join(COUNTRY_SETUP_PATH);
    std::fs::create_dir_all(&working_dir_country_setup_path).unwrap();
    write_country_setup_to_pdx_script(
        current_countries,
        working_dir_country_setup_path,
        game_country_setup_path,
    );

    let state_pop_path = working_dir.join("common/history/pops");
    std::fs::create_dir_all(&state_pop_path).unwrap();
    write_state_pops_to_pdx_script(&current_state_map, &state_pop_path);
    overwrite_existing_pops(
        &state_pop_path,
        &game_folder.join("game/common/history/pops"),
    );

    let state_buildings_path = working_dir.join("common/history/buildings");
    std::fs::create_dir_all(&state_buildings_path).unwrap();
    write_state_buildings_to_pdx_script(&current_state_map, &state_buildings_path);
    overwrite_existing_buildings(
        &state_buildings_path,
        &game_folder.join("game/common/history/buildings"),
    );

    let states_dir = working_dir.join("common/history/states");
    std::fs::create_dir_all(&states_dir).unwrap();
    write_states_to_pdx_script(states, current_state_map, states_dir);
    println!("Saved as mod in: {:?}", start.elapsed());
}

// This might be useful someday :')
// fn compare_sub_states(state1: &[SubStateProvince], state2: &[SubState]) -> bool {
//   let state1_map: HashMap<String, Vec<String>> = state1.iter().map(|sub_state| {
//     let mut sub_state_provinces = sub_state.provinces.clone();
//     sub_state_provinces.sort();
//     (sub_state.owner.clone(), sub_state_provinces)
//   }).collect();
//   let state2_map: HashMap<String, Vec<String>> = state2.iter().map(|sub_state| {
//     let mut sub_state_provinces = sub_state.provinces.clone();
//     sub_state_provinces.sort();
//     (sub_state.owner.clone(), sub_state_provinces)
//   }).collect();

//   state1_map == state2_map
// }

fn write_country_setup_to_pdx_script(
    current_countries: Vec<Country>,
    working_dir_country_setup_path: PathBuf,
    game_country_setup_path: PathBuf,
) {
    let country_setup_map = CountrySetup::parse_map_from(game_country_setup_path.clone());
    let unprocessed_setup_map = CountrySetup::parse_map_unprocessed_values(game_country_setup_path);
    let unprocessed_working_dir_setup_map =
        CountrySetup::parse_map_unprocessed_values(working_dir_country_setup_path.clone());

    current_countries.iter().for_each(|country| {
        let save_path =
            working_dir_country_setup_path.join(format!("{}.txt", country.name.to_lowercase()));
        if save_path.exists() {
            std::fs::remove_file(&save_path).unwrap();
        }

        let is_country_setup_changed = match country_setup_map.get(&country.name) {
            Some(game_country_setup) => {
                country.setup != *game_country_setup
                    || match unprocessed_working_dir_setup_map.get(&country.name) {
                        Some(unprocessed_working_dir_setup) => {
                            unprocessed_working_dir_setup.trim_start()
                                != unprocessed_setup_map
                                    .get(&country.name)
                                    .unwrap()
                                    .trim_start()
                        }
                        None => false,
                    }
            }
            None => true,
        };

        if is_country_setup_changed {
            let unparsed_script = match unprocessed_working_dir_setup_map.get(&country.name) {
                Some(script) => script,
                None => match unprocessed_setup_map.get(&country.name) {
                    Some(script) => script,
                    None => "  ",
                },
            };
            let mut country_setup_script = String::new();
            country_setup_script.push_str("COUNTRIES = {\n");
            country_setup_script.push_str(&format!("  c:{} = ", country.name));
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
    })
}

fn write_states_to_pdx_script(
    game_states: Vec<State>,
    current_state_map: HashMap<String, Vec<SubState>>,
    path: PathBuf,
) {
    let mut pdx_script = String::new();

    pdx_script.push_str("STATES = {\n");
    game_states.iter().for_each(|state| {
        pdx_script.push_str(&format!("  {} = ", state.name));
        pdx_script.push_str("{\n");

        current_state_map
            .get(&state.name)
            .unwrap()
            .iter()
            .for_each(|sub_state| {
                pdx_script.push_str("    create_state = {\n");
                pdx_script.push_str(&format!("      country = c:{}\n", sub_state.owner));

                pdx_script.push_str("      owned_provinces = { ");
                sub_state.provinces.iter().for_each(|province| {
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

fn write_state_pops_to_pdx_script(current_state_map: &HashMap<String, Vec<SubState>>, path: &Path) {
    let mut pdx_script = String::new();

    pdx_script.push_str("POPS = {\n");
    current_state_map
        .iter()
        .for_each(|(state_name, sub_states)| {
            pdx_script.push_str(&format!("  {} = ", state_name));
            pdx_script.push_str("{\n");
            sub_states.iter().for_each(|sub_state| {
                pdx_script.push_str(&format!("    region_state:{} = ", sub_state.owner));
                pdx_script.push_str("{\n");
                sub_state.pops.iter().for_each(|pop| {
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

fn write_state_buildings_to_pdx_script(
    current_state_map: &HashMap<String, Vec<SubState>>,
    path: &Path,
) {
    let mut pdx_script = String::new();

    pdx_script.push_str("BUILDINGS = {\n");
    current_state_map
        .iter()
        .for_each(|(state_name, sub_states)| {
            let mut conditioned_sub_state_buildings: Vec<SubState> = vec![];

            pdx_script.push_str(&format!("  {} = ", state_name));
            pdx_script.push_str("{\n");
            sub_states.iter().for_each(|sub_state| {
                pdx_script.push_str(&format!("    region_state:{} = ", sub_state.owner));
                pdx_script.push_str("{\n");
                for building in &sub_state.state_buildings {
                    if building.condition.is_some() {
                        let mut conditioned_sub_state = sub_state.clone();
                        conditioned_sub_state.state_buildings = vec![building.clone()];
                        conditioned_sub_state_buildings.push(conditioned_sub_state);
                        continue;
                    }
                    pdx_script.push_str("      create_building = {\n");
                    pdx_script.push_str(&format!("        building=\"{}\"\n", building.name));
                    if let Some(level) = building.level {
                        pdx_script.push_str(&format!("        level={}\n", level));
                    }
                    if let Some(reserves) = building.reserves {
                        pdx_script.push_str(&format!("        reserves={}\n", reserves));
                    }
                    if let Some(activate_production_methods) = &building.activate_production_methods
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

            pdx_script.push_str(
                parse_building_edge_case_conditional_to_string(
                    state_name.clone(),
                    conditioned_sub_state_buildings,
                )
                .as_str(),
            );
        });
    pdx_script.push_str("}\n");
    std::fs::write(path.join("00_buildings.txt"), pdx_script).unwrap();
}

fn parse_building_edge_case_conditional_to_string(
    state_name: String,
    sub_states_with_conditional_buildings: Vec<SubState>,
) -> String {
    let mut pdx_script = String::new();

    sub_states_with_conditional_buildings
        .iter()
        .for_each(|sub_state| {
            pdx_script.push_str("  if = {\n");
            pdx_script.push_str("    limit = {\n");
            let condition = sub_state.state_buildings[0]
                .condition
                .as_ref()
                .unwrap()
                .as_array();
            condition.unwrap().iter().for_each(|item| {
                pdx_script.push_str(&format!("      {} = {}\n", item[0], item[1]));
            });
            pdx_script.push_str("    }\n");
            pdx_script.push_str(&format!("    {} = ", state_name));
            pdx_script.push_str("{\n");
            pdx_script.push_str(&format!("      region_state:{} = ", sub_state.owner));
            pdx_script.push_str("{\n");
            sub_state.state_buildings.iter().for_each(|building| {
                pdx_script.push_str("        create_building = {\n");
                pdx_script.push_str(&format!("          building=\"{}\"\n", building.name));
                if let Some(level) = building.level {
                    pdx_script.push_str(&format!("          level={}\n", level));
                }
                if let Some(reserves) = building.reserves {
                    pdx_script.push_str(&format!("          reserves={}\n", reserves));
                }
                if let Some(activate_production_methods) = &building.activate_production_methods {
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
        });

    pdx_script
}

fn overwrite_existing_buildings(path: &Path, game_buildings_path: &PathBuf) {
    for entry in std::fs::read_dir(game_buildings_path).unwrap() {
        let entry_name = entry.unwrap().file_name().to_str().unwrap().to_string();
        std::fs::write(path.join(entry_name), "").unwrap();
    }
}
