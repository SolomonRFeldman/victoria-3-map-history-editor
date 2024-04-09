use std::path::PathBuf;
use image_dds::image::Rgba;
use tauri::{Manager, WindowMenuEvent};
use crate::{province_map_to_geojson::province_map_to_geojson, dds_to_png::DdsToPng};

const FLATMAP_PATH: &str = "game/dlc/dlc004_voice_of_the_people/gfx/map/textures/flatmap_votp.dds";
const LAND_MASK_PATH: &str = "game/gfx/map/textures/land_mask.dds";
const FLATMAP_OVERLAY_PATH: &str = "game/dlc/dlc004_voice_of_the_people/gfx/map/textures/flatmap_overlay_votp.dds";
const PROVINCE_PATH: &str = "game/map_data/provinces.png";

pub struct GameFolder {
  pub folder_path: PathBuf,
}

impl GameFolder {
  pub fn load(&self, event: WindowMenuEvent) {
    self.load_flatmap(&event);
    self.load_land_mask(&event);
    self.load_flatmap_overlay(&event);
    self.load_provinces(&event);
  }

  fn load_flatmap(&self, event: &WindowMenuEvent) {
    let dds_to_png = DdsToPng { dds_file_path: self.flatmap() };

    match dds_to_png.cache(cache_dir(event)) {
      Ok(_) => handle_send_map(event, "load-flatmap"),
      Err(_) => println!("Flatmap already in cache"),
    };
  }

  fn load_land_mask(&self, event: &WindowMenuEvent) {
    let dds_to_png = DdsToPng { dds_file_path: self.land_mask() };

    if !dds_to_png.exists_in_cache(cache_dir(event)) {
      let mut png_buffer = dds_to_png.dds_to_buffer();
      for pixel in png_buffer.pixels_mut() { if pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0 { *pixel = Rgba([0, 0, 0, 0]); } }

      dds_to_png.write_image(png_buffer, dds_to_png.png_file_path(cache_dir(event))).unwrap();
      handle_send_map(event, "load-land-mask");
    } else {
      println!("Land mask already in cache");
    }
  }

  fn load_flatmap_overlay(&self, event: &WindowMenuEvent) {
    let dds_to_png = DdsToPng { dds_file_path: self.flatmap_overlay() };

    match dds_to_png.cache(cache_dir(event)) {
      Ok(_) => handle_send_map(event, "load-flatmap-overlay"),
      Err(_) => println!("Flatmap overlay already in cache"),
    };
  }

  fn load_provinces(&self, event: &WindowMenuEvent) {
    match event.window().emit("load-province-data", province_map_to_geojson(self.provinces())) {
      Ok(_) => println!("Sent load-province-data to frontend"),
      Err(e) => println!("Failed to send load-province-data to frontend: {:?}", e),
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
