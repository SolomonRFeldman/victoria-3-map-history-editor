use std::{collections::HashMap, path::PathBuf};
use serde::{Deserialize, Serialize};

use crate::pdx_script_parser::parse_script;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pop {
  pub culture: String,
  pub religion: Option<String>,
  pub size: i64,
  pub pop_type: Option<String>,
}

#[derive(Debug)]
pub struct StatePopulation {
  pub pops: Vec<Pop>,
}

pub fn get_state_populations(state_pop_path: PathBuf) -> HashMap<String, StatePopulation> {
  let mut state_populations: HashMap<String, StatePopulation> = HashMap::new();

  for entry in std::fs::read_dir(state_pop_path).unwrap() {
    let entry = entry.unwrap().path();

    let parsed_state_pops = parse_script(&std::fs::read_to_string(entry).unwrap());
    parsed_state_pops[0][1].as_array().unwrap().iter().for_each(|state_pops| {
      let state_name = state_pops[0].as_str().unwrap().to_string();

      state_pops[1].as_array().unwrap().iter().for_each(|raw_sub_state_pops| {
        let country_name = raw_sub_state_pops[0].as_str().unwrap().strip_prefix("region_state:").unwrap();

        let parsed_sub_state_pops: Vec<Pop> = raw_sub_state_pops[1].as_array().unwrap().iter().map(|raw_pop| {
          let pop = raw_pop[1].as_array().unwrap();
          let culture = pop.iter().find(|item| item[0] == "culture").unwrap()[1].as_str().unwrap().to_string();
          let size: i64 = pop.iter().find(|item| item[0] == "size").unwrap()[1].as_str().unwrap().parse().unwrap();
          let religion = match pop.iter().find(|item| item[0] == "religion") {
            Some(religion) => Some(religion[1].as_str().unwrap().to_string()),
            None => None,
          };
          let pop_type = match pop.iter().find(|item| item[0] == "pop_type") {
            Some(pop_type) => Some(pop_type[1].as_str().unwrap().to_string()),
            None => None,
          };

          Pop {
            culture,
            religion,
            size,
            pop_type,
          }
        }).collect();

        state_populations.insert(format!("{}:{}", country_name, state_name), StatePopulation {
          pops: parsed_sub_state_pops,
        });
      });
    });

  };

  state_populations
}
