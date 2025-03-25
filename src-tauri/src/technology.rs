use std::{collections::HashMap, path::PathBuf};

use jomini::text::de::from_utf8_reader;
use serde::{Deserialize, Serialize};
use tauri::{Manager, Window};

use crate::cache_config::CacheConfig;

const TECHNOLOGIES_PATH: &str = "common/technology/technologies";

#[derive(Debug, Deserialize, Serialize)]
pub struct Technology {
    name: String,
    era: String,
    category: String,
}

#[derive(Deserialize, Serialize)]
struct RawTechnology {
    era: String,
    category: String,
}

impl Technology {
    fn parse_from(path: PathBuf) -> Vec<Technology> {
        let mut technologies: Vec<Technology> = Vec::new();

        for entry in std::fs::read_dir(path.join(TECHNOLOGIES_PATH)).unwrap() {
            let entry = entry.unwrap().path();
            if entry.extension().unwrap() != "txt" {
                continue;
            };

            let raw_technologies: HashMap<String, RawTechnology> =
                from_utf8_reader(&*std::fs::read(entry).unwrap()).unwrap();

            for (name, RawTechnology { era, category }) in raw_technologies {
                technologies.push(Technology {
                    name,
                    era,
                    category,
                });
            }
        }

        technologies
    }

    pub fn parse_from_game_folder(window: Window) -> Vec<Technology> {
        let cache_dir = window.app_handle().path().app_cache_dir().unwrap();
        let cache_config: CacheConfig = CacheConfig::get_config(cache_dir.join("config.json"));

        Self::parse_from(cache_config.game_folder.unwrap().join("game"))
    }
}
