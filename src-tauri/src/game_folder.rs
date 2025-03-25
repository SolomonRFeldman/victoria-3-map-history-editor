use crate::{
    cache_config::CacheConfig,
    country_definition::CountryDefinition,
    country_setup::CountrySetup,
    dds_to_png::DdsToPng,
    get_countries::get_countries,
    get_state_buildings::get_state_buildings,
    get_state_populations::get_state_populations,
    get_states::get_states,
    province_map_to_geojson::{
        country_map_to_geojson, province_map_to_geojson, state_map_to_geojson,
    },
};
use image_dds::image::Rgba;
use std::{collections::HashMap, path::PathBuf};
use tauri::{AppHandle, Emitter, Manager};

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
        self.load_states();
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

    fn load_states(&self) {
        let states = get_states(self.states());
        let state_coords = state_map_to_geojson(
            self.provinces(),
            cache_dir(&self.app_handle).join("states.png"),
            states,
        );
        std::fs::write(
            cache_dir(&self.app_handle).join("states.json"),
            serde_json::to_string(&state_coords).unwrap(),
        )
        .unwrap();

        match self.app_handle.emit("load-state-coords", true) {
            Ok(_) => println!("Sent load-state-coords to frontend"),
            Err(e) => println!("Failed to send load-state-coords to frontend: {:?}", e),
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
        let countries_with_coords = country_map_to_geojson(
            cache_dir(&self.app_handle).join("states.png"),
            cache_dir(&self.app_handle).join("countries.png"),
            countries.clone(),
        );
        std::fs::write(
            cache_dir(&self.app_handle).join("countries.json"),
            serde_json::to_string(&countries_with_coords).unwrap(),
        )
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
