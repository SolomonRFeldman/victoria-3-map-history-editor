use std::{fs::File, path::PathBuf};
use ddsfile::Dds;
use image::{codecs::png::PngEncoder, ExtendedColorType, ImageEncoder};
use image_dds::image::{ImageBuffer, Rgba};

pub struct DdsToPng {
  pub dds_file_path: PathBuf,
}

// TO-DO: Split up the main encode function. Implement error handling. Split up the cache function.
impl DdsToPng {
  pub fn dds_to_buffer(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let dds_file = Dds::read(File::open(&self.dds_file_path).unwrap()).unwrap();
    image_dds::image_from_dds(&dds_file, 0).unwrap()
  }

  pub fn cache(&self, cache_dir: PathBuf) -> Result<(), ()> {
    if self.exists_in_cache(cache_dir.clone()) {
      return Err(());
    }
    
    let image_buffer = self.dds_to_buffer();
    match self.write_image(image_buffer, self.png_file_path(cache_dir)) {
      Ok(_) => {},
      Err(e) => {
        println!("Failed to encode image to PNG: {:?}", e);
      },
    };
    Ok(())
  }

  pub fn write_image(&self, image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>, path: PathBuf) -> Result<(), std::io::Error> {
    let mut image_file = File::create(path).unwrap();

    match PngEncoder::new(&mut image_file).write_image(&image_buffer.clone().into_raw(), image_buffer.width(), image_buffer.height(), ExtendedColorType::Rgba8) {
      Ok(_) => {},
      Err(e) => {
        println!("Failed to encode image to PNG: {:?}", e);
      },
    };
    Ok(())
  }

  pub fn exists_in_cache(&self, cache_dir: PathBuf) -> bool {
    self.png_file_path(cache_dir).exists()
  }

  pub fn png_file_path(&self, cache_dir: PathBuf) -> PathBuf {
    let mut png_file_path = cache_dir.join(self.dds_file_path.file_name().unwrap());
    png_file_path.set_extension("png");
    png_file_path
  }
}
