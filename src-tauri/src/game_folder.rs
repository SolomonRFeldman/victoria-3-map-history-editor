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
    let mut province_borders: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
    let image_height = provinces.height() as i32;

    for (x, y, pixel) in provinces.enumerate_pixels() {
      let province_id = format!("x{:02X}{:02X}{:02X}", pixel[0], pixel[1], pixel[2]);

      let x = x as i32;
      let y = y as i32;

      let top: (i32, i32, i32, i32, i32, i32, i32, i32) = (x, y - 1, 0, 0, 2, 0, 1, 0);
      let left: (i32, i32, i32, i32, i32, i32, i32, i32)  = (x - 1, y, 0, 0, 0, 2, 0, 1);
      let right: (i32, i32, i32, i32, i32, i32, i32, i32)  = (x + 1, y, 2, 2, 2, 0, 2, 1);
      let bottom: (i32, i32, i32, i32, i32, i32, i32, i32)  = (x, y + 1, 0, 2, 2, 2, 1, 2);

      let neighbors = [
        bottom, top, left, right
      ];

      neighbors.iter().for_each(|&neighbor| {
        if neighbor.0 < 0 || neighbor.1 < 0 || neighbor.0 >= provinces.width() as i32 || neighbor.1 >= provinces.height() as i32 || provinces.get_pixel(neighbor.0 as u32, neighbor.1 as u32) != pixel {
          province_borders.entry(province_id.clone()).or_default().push(((x * 2) + neighbor.2, (image_height * 2) - ((y * 2) + neighbor.3)));
          province_borders.entry(province_id.clone()).or_default().push(((x * 2) + neighbor.4, (image_height * 2) - ((y * 2) + neighbor.5)));
          province_borders.entry(province_id.clone()).or_default().push(((x * 2) + neighbor.6, (image_height * 2) - ((y * 2) + neighbor.7)));
        }
      });
    }

    #[derive(Clone, Serialize)]
    struct Province {
      name: String,
      coords: Vec<(f32, f32)>,
    }

    let geojson_provinces = province_borders.iter().map(|(hex_color, coords)| {
      let geo_json_coords = border_to_geojson_coords(coords.clone())
        .iter()
        .map(|&(x, y)| (x as f32 / 2 as f32, y as f32 / 2 as f32))
        .collect::<Vec<(f32, f32)>>();

      Province { 
        name: hex_color.clone(), 
        coords: geo_json_coords
      }
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
