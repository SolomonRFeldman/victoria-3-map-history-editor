use std::{fs::File, path::PathBuf};
use base64::{engine::general_purpose, Engine};
use ddsfile::Dds;
use image::{codecs::png::PngEncoder, ExtendedColorType, ImageEncoder};

const TODO_ERROR: &str = "Figure out how to handle errors";

pub struct DdsToPng {
  pub dds_file_path: PathBuf,
}

// TO-DO: Split up the main encode function. Implement error handling.
impl DdsToPng {
  pub fn encode(&self) -> String {
    let dds_file = match File::open(&self.dds_file_path) {
      Ok(dds_file) => match Dds::read(dds_file) {
        Ok(decoder) => decoder,
        Err(e) => {
          println!("Failed to decode DDS file: {:?}", e);
          return TODO_ERROR.to_string();
        }
      },
      Err(e) => {
        println!("Failed to open DDS file: {:?}", e);
        return TODO_ERROR.to_string();
      }
    };

    let image_buffer = match image_dds::image_from_dds(&dds_file, 0) {
      Ok(image_buffer) => image_buffer,
      Err(e) => {
          println!("Failed to convert DDS file to image: {:?}", e);
          return TODO_ERROR.to_string();
      }
    };

    let mut png_buffer = Vec::new();
    match PngEncoder::new(&mut png_buffer).write_image(&image_buffer.clone().into_raw(), image_buffer.width(), image_buffer.height(), ExtendedColorType::Rgba8) {
      Ok(_) => {},
      Err(e) => {
        println!("Failed to encode image to PNG: {:?}", e);
        return TODO_ERROR.to_string();
      },
    };

    general_purpose::STANDARD.encode(&png_buffer)
  }
}
