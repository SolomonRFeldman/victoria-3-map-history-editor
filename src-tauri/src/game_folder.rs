use std::path::PathBuf;
use tauri::{Manager, WindowMenuEvent};
use crate::dds_to_png::DdsToPng;

const FLATMAP_PATH: &str = "game/dlc/dlc004_voice_of_the_people/gfx/map/textures/flatmap_votp.dds";
const FLATMAP_OVERLAY_PATH: &str = "game/dlc/dlc004_voice_of_the_people/gfx/map/textures/flatmap_overlay_votp.dds";

pub struct GameFolder {
  pub folder_path: PathBuf,
}

impl GameFolder {
  pub fn load(&self, event: WindowMenuEvent) {
    self.load_flatmap(&event);
    self.load_flatmap_overlay(&event);
  }

  fn load_flatmap(&self, event: &WindowMenuEvent) {
    let dds_to_png = DdsToPng { dds_file_path: self.flatmap() };

    match dds_to_png.cache(cache_dir(event)) {
      Ok(_) => handle_send_map(event, "load-flatmap"),
      Err(e) => println!("Failed to cache flatmap: {:?}", e),
    };
  }

  fn load_flatmap_overlay(&self, event: &WindowMenuEvent) {
    let dds_to_png = DdsToPng { dds_file_path: self.flatmap_overlay() };

    match dds_to_png.cache(cache_dir(event)) {
      Ok(_) => handle_send_map(event, "load-flatmap-overlay"),
      Err(e) => println!("Failed to cache flatmap: {:?}", e),
    };
  }

  fn flatmap(&self) -> PathBuf {
    self.folder_path.join(PathBuf::from(FLATMAP_PATH))
  }

  fn flatmap_overlay(&self) -> PathBuf {
    self.folder_path.join(PathBuf::from(FLATMAP_OVERLAY_PATH))
  }
}

fn handle_send_map(event: &WindowMenuEvent, event_id: &str) {
  match event.window().emit(event_id, true) {
    Ok(_) => println!("Sent {:?} to frontend", event_id),
    Err(e) => println!("Failed to send {:?} to frontend: {:?}", event_id, e),
  }
}

fn cache_dir(event: &WindowMenuEvent) -> PathBuf {
  event.window().app_handle().path_resolver().app_cache_dir().unwrap()
}
