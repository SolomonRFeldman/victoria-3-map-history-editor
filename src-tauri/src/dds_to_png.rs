use std::{fs::File, path::PathBuf};
use ddsfile::Dds;
use image::{codecs::png::PngEncoder, ExtendedColorType, ImageEncoder};
use image_dds::image::{ImageBuffer, Rgba};

pub struct DdsToPng {
  pub dds_file_path: PathBuf,
}

// TO-DO: Split up the main encode function. Implement error handling.
impl DdsToPng {
  fn dds_to_buffer(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let dds_file = Dds::read(File::open(&self.dds_file_path).unwrap()).unwrap();
    image_dds::image_from_dds(&dds_file, 0).unwrap()
  }

  pub fn cache(&self, cache_dir: PathBuf) -> Result<(), std::io::Error> {
    let mut cache_file_path = cache_dir.join(self.dds_file_path.file_name().unwrap());
    cache_file_path.set_extension("png");

    let mut cached_image = match File::create_new(cache_file_path) {
      Ok(file) => file,
      Err(e) => {
        println!("Failed to create cache file: {:?}", e);
        return Err(e);
      },
    };
    
    let image_buffer = self.dds_to_buffer();
    match PngEncoder::new(&mut cached_image).write_image(&image_buffer.clone().into_raw(), image_buffer.width(), image_buffer.height(), ExtendedColorType::Rgba8) {
      Ok(_) => {},
      Err(e) => {
        println!("Failed to encode image to PNG: {:?}", e);
      },
    };
    Ok(())
  }
}
