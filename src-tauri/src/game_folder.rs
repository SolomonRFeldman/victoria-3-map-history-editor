use crate::{
    cache_config::CacheConfig,
    country_definition::CountryDefinition,
    country_setup::CountrySetup,
    dds_to_png::DdsToPng,
    get_countries::get_countries,
    get_state_buildings::get_state_buildings,
    get_state_populations::get_state_populations,
    get_states::get_states,
    models::{
        building, building_ownership,
        country::{self, Border, Color},
        country_ownership, pop,
        state::{self, Provinces},
    },
    province_map_to_geojson::{
        country_map_to_geojson, province_map_to_geojson, state_map_to_geojson,
    },
};
use image_dds::image::Rgba;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};
use std::{collections::HashMap, path::PathBuf};
use tauri::{async_runtime::block_on, AppHandle, Emitter, Manager};

const FLATMAP_PATH: &str = "game/dlc/dlc004_voice_of_the_people/gfx/map/textures/flatmap_votp.dds";
const LAND_MASK_PATH: &str = "game/gfx/map/textures/land_mask.dds";
const FLATMAP_OVERLAY_PATH: &str =
    "game/dlc/dlc004_voice_of_the_people/gfx/map/textures/flatmap_overlay_votp.dds";
const PROVINCE_PATH: &str = "game/map_data/provinces.png";
pub const STATES_PATH: &str = "game/common/history/states/00_states.txt";
pub const STATE_POPS_PATH: &str = "game/common/history/pops";
const STATE_BUILDINGS_PATH: &str = "game/common/history/buildings";
pub const COUNTRY_DEFINITIONS_PATH: &str = "common/country_definitions";
pub const COUNTRY_SETUP_PATH: &str = "common/history/countries";

pub struct GameFolder {
    pub folder_path: PathBuf,
    pub app_handle: AppHandle,
}

impl GameFolder {
    pub fn load(&self) {
        self.write_path_to_config();
        self.load_flatmap();
        self.load_land_mask();
        self.load_flatmap_overlay();
        self.load_countries();
        self.load_provinces();
    }

    fn write_path_to_config(&self) {
        let config_path = cache_dir(&self.app_handle).join("config.json");

        let mut config: CacheConfig = match std::fs::read_to_string(&config_path) {
            Ok(config) => serde_json::from_str(&config).unwrap(),
            Err(_) => CacheConfig::new(),
        };
        config.game_folder = Some(self.folder_path.clone());
        std::fs::write(config_path, serde_json::to_string(&config).unwrap()).unwrap();
    }

    fn load_flatmap(&self) {
        let dds_to_png = DdsToPng {
            dds_file_path: self.flatmap(),
        };

        match dds_to_png.cache(cache_dir(&self.app_handle)) {
            Ok(_) => handle_send_map(&self.app_handle, "load-flatmap"),
            Err(_) => println!("Flatmap already in cache"),
        };
    }

    fn load_land_mask(&self) {
        let dds_to_png = DdsToPng {
            dds_file_path: self.land_mask(),
        };

        if !dds_to_png.exists_in_cache(cache_dir(&self.app_handle)) {
            let mut png_buffer = dds_to_png.dds_to_buffer();
            for pixel in png_buffer.pixels_mut() {
                if pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0 {
                    *pixel = Rgba([0, 0, 0, 0]);
                }
            }

            dds_to_png
                .write_image(
                    png_buffer,
                    dds_to_png.png_file_path(cache_dir(&self.app_handle)),
                )
                .unwrap();
            handle_send_map(&self.app_handle, "load-land-mask");
        } else {
            println!("Land mask already in cache");
        }
    }

    fn load_flatmap_overlay(&self) {
        let dds_to_png = DdsToPng {
            dds_file_path: self.flatmap_overlay(),
        };

        match dds_to_png.cache(cache_dir(&self.app_handle)) {
            Ok(_) => handle_send_map(&self.app_handle, "load-flatmap-overlay"),
            Err(_) => println!("Flatmap overlay already in cache"),
        };
    }

    fn load_provinces(&self) {
        match cache_dir(&self.app_handle).join("provinces.json").exists() {
            true => {}
            false => {
                let provinces = province_map_to_geojson(self.provinces());
                std::fs::write(
                    cache_dir(&self.app_handle).join("provinces.json"),
                    serde_json::to_string(&provinces).unwrap(),
                )
                .unwrap();
            }
        };

        match self.app_handle.emit("load-province-coords", true) {
            Ok(_) => println!("Sent load-province-coords to frontend"),
            Err(e) => println!("Failed to send load-province-coords to frontend: {:?}", e),
        }
    }

    fn load_countries(&self) {
        let countries = get_countries(
            get_states(self.states()),
            get_state_populations(self.state_pops()),
            get_state_buildings(self.state_buildings()),
            self.country_definitions(),
            self.country_setups(),
        );
        let db = self.app_handle.state::<DatabaseConnection>().inner();
        block_on(Migrator::down(db, None)).unwrap();
        block_on(Migrator::up(db, None)).unwrap();
        let countries_with_coords = country_map_to_geojson(
            cache_dir(&self.app_handle).join("states.png"),
            cache_dir(&self.app_handle).join("countries.png"),
            countries.clone(),
        );
        let countries_to_insert: Vec<country::ActiveModel> = countries_with_coords
            .iter()
            .map(|country| country::ActiveModel {
                tag: Set(country.name.clone()),
                color: Set(Color(country.color)),
                setup: Set(country.setup.clone()),
                border: Set(Border(
                    country
                        .coordinates
                        .iter()
                        .map(|polygon| polygon.iter().map(|&(x, y)| (x, y)).collect())
                        .collect(),
                )),
                ..Default::default()
            })
            .collect();
        block_on(country::Entity::insert_many(countries_to_insert).exec(db)).unwrap();

        let inserted_countries = block_on(
            country::Entity::find()
                .into_model::<country::WithoutBorder>()
                .all(db),
        )
        .unwrap();

        let states = get_states(self.states());
        let state_coords = state_map_to_geojson(
            self.provinces(),
            cache_dir(&self.app_handle).join("states.png"),
            states,
        );

        let states_to_insert: Vec<state::ActiveModel> = countries
            .iter()
            .flat_map(|country| {
                let country_id = inserted_countries
                    .iter()
                    .find(|inserted_country| inserted_country.tag == country.name)
                    .unwrap()
                    .id;
                country
                    .states
                    .iter()
                    .map(|state| state::ActiveModel {
                        name: Set(state.name.trim_start_matches("s:").to_string()),
                        country_id: Set(country_id),
                        provinces: Set(Provinces(state.provinces.clone())),
                        border: Set(Border(
                            state_coords
                                .get(&format!("{}:{}", country.name.clone(), state.name.clone()))
                                .unwrap()
                                .clone()
                                .iter()
                                .map(|polygon| polygon.iter().map(|&(x, y)| (x, y)).collect())
                                .collect(),
                        )),
                        ..Default::default()
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        block_on(state::Entity::insert_many(states_to_insert).exec(db)).unwrap();

        let inserted_states = block_on(
            state::Entity::find()
                .into_model::<state::WithoutBorder>()
                .all(db),
        )
        .unwrap();

        let country_map: HashMap<i32, &country::WithoutBorder> = inserted_countries
            .iter()
            .map(|country| (country.id, country))
            .collect();

        let state_pops = get_state_populations(self.state_pops());

        let pops_to_insert: Vec<pop::ActiveModel> = inserted_states
            .iter()
            .flat_map(|state| {
                let country_tag = country_map.get(&state.country_id).unwrap().tag.clone();
                let pops = state_pops
                    .get(&format!("{}:s:{}", country_tag, state.name))
                    .unwrap()
                    .pops
                    .clone();
                pops.iter()
                    .map(|pop| pop::ActiveModel {
                        state_id: Set(state.id),
                        culture: Set(pop.culture.clone()),
                        religion: Set(pop.religion.clone()),
                        size: Set(pop.size),
                        pop_type: Set(pop.pop_type.clone()),
                        ..Default::default()
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        block_on(pop::Entity::insert_many(pops_to_insert).exec(db)).unwrap();

        let state_buildings = get_state_buildings(self.state_buildings());

        let buildings_to_insert: Vec<building::ActiveModel> = inserted_states
            .iter()
            .flat_map(|state| {
                let country_tag = country_map.get(&state.country_id).unwrap().tag.clone();
                if let Some(state_buildings) =
                    state_buildings.get(&format!("{}:s:{}", country_tag, state.name))
                {
                    state_buildings
                        .iter()
                        .map(|building| building::ActiveModel {
                            state_id: Set(state.id),
                            name: Set(building.name.clone()),
                            level: Set(building.level),
                            reserves: Set(building.reserves),
                            activate_production_methods: Set(building::ActivateProductionMethods(
                                building.activate_production_methods.clone(),
                            )),
                            condition: Set(building.condition.clone()),
                            ..Default::default()
                        })
                        .collect::<Vec<_>>()
                } else {
                    vec![]
                }
            })
            .collect();
        block_on(building::Entity::insert_many(buildings_to_insert).exec(db)).unwrap();

        let inserted_buildings = block_on(building::Entity::find().all(db)).unwrap();

        let mut country_ownerships_to_insert: Vec<country_ownership::ActiveModel> = vec![];
        let mut building_ownerships_to_insert: Vec<building_ownership::ActiveModel> = vec![];
        for country in countries {
            let country_id = inserted_countries
                .iter()
                .find(|inserted_country| inserted_country.tag == country.name)
                .unwrap()
                .id;
            for state in &country.states {
                let state_id = inserted_states
                    .iter()
                    .find(|inserted_state| {
                        inserted_state.name == state.name.trim_start_matches("s:")
                            && inserted_state.country_id == country_id
                    })
                    .unwrap()
                    .id;
                for building in &state.state_buildings {
                    let building_id = inserted_buildings
                        .iter()
                        .find(|inserted_building| {
                            inserted_building.name == building.name
                                && inserted_building.state_id == state_id
                        })
                        .unwrap()
                        .id;
                    if let Some(ownership) = &building.ownership {
                        for country_ownership in ownership.countries.clone() {
                            let country_id = inserted_countries
                                .iter()
                                .find(|inserted_country| {
                                    inserted_country.tag
                                        == country_ownership.country.trim_start_matches("c:")
                                })
                                .unwrap()
                                .id;
                            country_ownerships_to_insert.push(country_ownership::ActiveModel {
                                building_id: Set(building_id),
                                country_id: Set(country_id),
                                levels: Set(country_ownership.levels),
                                ..Default::default()
                            });
                        }
                        for building_ownership in ownership.buildings.clone() {
                            let country_id = inserted_countries
                                .iter()
                                .find(|inserted_country| {
                                    inserted_country.tag
                                        == building_ownership.country.trim_start_matches("c:")
                                })
                                .unwrap()
                                .id;
                            let state_id = inserted_states
                                .iter()
                                .find(|inserted_state| {
                                    inserted_state.name == building_ownership.region
                                        && inserted_state.country_id == country_id
                                })
                                .unwrap()
                                .id;
                            building_ownerships_to_insert.push(building_ownership::ActiveModel {
                                building_id: Set(building_id),
                                state_id: Set(state_id),
                                owner_type: Set(building_ownership.type_),
                                levels: Set(building_ownership.levels),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }
        block_on(country_ownership::Entity::insert_many(country_ownerships_to_insert).exec(db))
            .unwrap();
        block_on(building_ownership::Entity::insert_many(building_ownerships_to_insert).exec(db))
            .unwrap();

        match self.app_handle.emit("load-country-data", true) {
            Ok(_) => println!("Sent load-country-data to frontend"),
            Err(e) => println!("Failed to send load-country-data to frontend: {:?}", e),
        }
    }

    fn flatmap(&self) -> PathBuf {
        self.folder_path.join(PathBuf::from(FLATMAP_PATH))
    }

    fn land_mask(&self) -> PathBuf {
        self.folder_path.join(PathBuf::from(LAND_MASK_PATH))
    }

    fn flatmap_overlay(&self) -> PathBuf {
        self.folder_path.join(PathBuf::from(FLATMAP_OVERLAY_PATH))
    }

    fn provinces(&self) -> PathBuf {
        self.folder_path.join(PathBuf::from(PROVINCE_PATH))
    }

    fn states(&self) -> PathBuf {
        self.folder_path.join(PathBuf::from(STATES_PATH))
    }

    fn state_pops(&self) -> PathBuf {
        self.folder_path.join(PathBuf::from(STATE_POPS_PATH))
    }

    fn state_buildings(&self) -> PathBuf {
        self.folder_path.join(PathBuf::from(STATE_BUILDINGS_PATH))
    }

    fn game_path(&self) -> PathBuf {
        self.folder_path.join(PathBuf::from("game"))
    }

    fn country_definitions(&self) -> HashMap<String, CountryDefinition> {
        CountryDefinition::parse_map_from(
            self.game_path()
                .join(PathBuf::from(COUNTRY_DEFINITIONS_PATH)),
        )
    }

    fn country_setups(&self) -> HashMap<String, CountrySetup> {
        CountrySetup::parse_map_from(self.game_path().join(PathBuf::from(COUNTRY_SETUP_PATH)))
    }
}

fn handle_send_map(app_handle: &AppHandle, event_id: &str) {
    match app_handle.emit(event_id, true) {
        Ok(_) => println!("Sent {:?} to frontend", event_id),
        Err(e) => println!("Failed to send {:?} to frontend: {:?}", event_id, e),
    }
}

fn cache_dir(app_handle: &AppHandle) -> PathBuf {
    app_handle.path().app_cache_dir().unwrap()
}
