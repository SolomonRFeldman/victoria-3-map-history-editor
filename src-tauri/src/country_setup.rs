use std::{collections::HashMap, path::PathBuf};

use jomini::{text::de::from_utf8_reader, JominiDeserialize};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CountrySetup {
    base_tech: Option<String>,
    technologies_researched: Vec<String>,
}

fn default_false() -> bool {
    false
}
#[derive(JominiDeserialize)]
struct RawCountrySetup {
    #[jomini(default = "default_false")]
    effect_starting_technology_tier_1_tech: bool,
    #[jomini(default = "default_false")]
    effect_starting_technology_tier_2_tech: bool,
    #[jomini(default = "default_false")]
    effect_starting_technology_tier_3_tech: bool,
    #[jomini(default = "default_false")]
    effect_starting_technology_tier_4_tech: bool,
    #[jomini(default = "default_false")]
    effect_starting_technology_tier_5_tech: bool,
    #[jomini(default = "default_false")]
    effect_starting_technology_tier_6_tech: bool,
    #[jomini(default = "default_false")]
    effect_starting_technology_tier_7_tech: bool,
    #[jomini(alias = "add_technology_researched", duplicated)]
    technologies_researched: Vec<String>,
}

#[derive(JominiDeserialize)]
struct CountrySetupFile {
    #[jomini(alias = "COUNTRIES")]
    countries: HashMap<String, RawCountrySetup>,
}

impl CountrySetup {
    pub fn new() -> CountrySetup {
        CountrySetup {
            base_tech: Some("tier_7".to_string()),
            technologies_researched: vec![],
        }
    }
    pub fn parse_map_from(path: PathBuf) -> HashMap<String, CountrySetup> {
        let mut country_setups: HashMap<String, CountrySetup> = HashMap::new();

        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap().path();
            if entry.extension().unwrap() != "txt" {
                continue;
            };

            let raw_country_setups: CountrySetupFile =
                from_utf8_reader(&*std::fs::read(entry).unwrap()).unwrap();

            raw_country_setups
                .countries
                .iter()
                .for_each(|(country, raw_country_setup)| {
                    let base_tech = if raw_country_setup.effect_starting_technology_tier_1_tech {
                        Some("tier_1".to_string())
                    } else if raw_country_setup.effect_starting_technology_tier_2_tech {
                        Some("tier_2".to_string())
                    } else if raw_country_setup.effect_starting_technology_tier_3_tech {
                        Some("tier_3".to_string())
                    } else if raw_country_setup.effect_starting_technology_tier_4_tech {
                        Some("tier_4".to_string())
                    } else if raw_country_setup.effect_starting_technology_tier_5_tech {
                        Some("tier_5".to_string())
                    } else if raw_country_setup.effect_starting_technology_tier_6_tech {
                        Some("tier_6".to_string())
                    } else if raw_country_setup.effect_starting_technology_tier_7_tech {
                        Some("tier_7".to_string())
                    } else {
                        None
                    };

                    let country_setup = CountrySetup {
                        base_tech,
                        technologies_researched: raw_country_setup.technologies_researched.clone(),
                    };

                    country_setups.insert(
                        country.trim_start_matches("c:").to_uppercase(),
                        country_setup,
                    );
                })
        }

        country_setups
    }
}
