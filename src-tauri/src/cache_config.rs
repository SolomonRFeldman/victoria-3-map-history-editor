use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CacheConfig {
  pub game_folder: Option<PathBuf>,
  pub working_dir: Option<PathBuf>,
}

impl CacheConfig {
  pub fn new() -> Self {
    Self {
      game_folder: None,
      working_dir: None,
    }
  }

  pub fn get_config(path: PathBuf) -> Self {
    let config = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&config).unwrap()
  }
}
