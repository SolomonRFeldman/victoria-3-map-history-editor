use std::path::PathBuf;
use image_dds::image::Rgba;
use image::io::Reader as ImageReader;
use serde::Serialize;
use tauri::{Manager, WindowMenuEvent};
use crate::{border_to_geojson_coords::border_to_geojson_coords, dds_to_png::DdsToPng};
use std::collections::HashMap;

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

  // TO-DO: should be broken up
  fn load_provinces(&self, event: &WindowMenuEvent) {
    let provinces = ImageReader::open(self.provinces()).unwrap().decode().unwrap().into_rgb8();
    let mut province_borders: HashMap<String, Vec<(u32, u32)>> = HashMap::new();

    for (x, y, pixel) in provinces.enumerate_pixels() {
      let top = provinces.get_pixel_checked(x, y.saturating_sub(1));
      let left = provinces.get_pixel_checked(x.saturating_sub(1), y);
      let right = provinces.get_pixel_checked(x + 1, y);
      let bottom = provinces.get_pixel_checked(x, y + 1);
      let top_left = provinces.get_pixel_checked(x.saturating_sub(1), y.saturating_sub(1));
      let top_right = provinces.get_pixel_checked(x + 1, y.saturating_sub(1));
      let bottom_right = provinces.get_pixel_checked(x + 1, y + 1);
      let bottom_left = provinces.get_pixel_checked(x.saturating_sub(1), y + 1);

      let neighbors = [
        top, left, right, bottom, top_left, top_right, bottom_right, bottom_left
      ];

      let is_border = neighbors.iter().any(|&neighbor| {
        if let Some(neighbor) = neighbor {
          neighbor != pixel || x == 0 || y == 0
        } else {
          false
        }
      });
      if is_border {
        let hex_color = format!("{:02X}{:02X}{:02X}", pixel[0], pixel[1], pixel[2]);
        province_borders.entry(format!("x{}", hex_color)).or_default().push((x, provinces.height() - y));
      }
    }

    #[derive(Clone, Serialize)]
    struct Province {
      name: String,
      coords: Vec<(i32, i32)>,
    }

    let geojson_provinces = province_borders.iter().map(|(hex_color, coords)| {
      Province { name: hex_color.clone(), coords: border_to_geojson_coords(coords.clone()) }
    }).collect::<Vec<Province>>();

    match event.window().emit("load-province-data", geojson_provinces) {
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
