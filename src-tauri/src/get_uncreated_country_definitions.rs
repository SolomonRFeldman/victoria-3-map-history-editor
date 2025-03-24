use std::collections::HashSet;
use tauri::{Manager, Window};

use crate::{
    cache_config::CacheConfig, country_definition::CountryDefinition,
    game_folder::COUNTRY_DEFINITIONS_PATH,
};

pub fn get_uncreated_country_definitions(
    window: Window,
    created_tag_set: HashSet<String>,
) -> Vec<CountryDefinition> {
    let country_definition_path = CacheConfig::get_config(
        window
            .app_handle()
            .path()
            .app_cache_dir()
            .unwrap()
            .join("config.json"),
    )
    .game_folder
    .unwrap()
    .join("game")
    .join(COUNTRY_DEFINITIONS_PATH);

    CountryDefinition::parse_from(country_definition_path)
        .iter()
        .filter(|definition| !created_tag_set.contains(&definition.tag))
        .cloned()
        .collect()
}
