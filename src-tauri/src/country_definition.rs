use std::{collections::HashMap, path::PathBuf};
use jomini::TextTape;

use crate::color_converter::ColorConverter;

#[derive(Debug, Clone)]
pub struct CountryDefinition {
  pub tag: String,
  pub color: (u8, u8, u8),
}

impl CountryDefinition {
  pub fn parse_from(path: PathBuf) -> HashMap<String, CountryDefinition> {
    let mut country_definitions: HashMap<String, CountryDefinition> = HashMap::new();

    for entry in std::fs::read_dir(path).unwrap() {
      let entry = entry.unwrap().path();
      if entry.extension().unwrap() != "txt" || entry.file_name().unwrap() == "99_dynamic.txt" { continue };

      let data = &std::fs::read(&entry).unwrap();
      let tape = TextTape::from_slice(data).unwrap();
      let reader = tape.utf8_reader();

      for (key, _op, value) in reader.fields() {
        let tag = key.read_str().to_string();

        for (key, _op, value) in value.read_object().unwrap().fields() {
          if key.read_str() == "color" {
            match value.read_array().unwrap().values().nth(1).unwrap().read_string() {
              Ok(_) => {
                let colors: (u8, u8, u8) = match value.read_array().unwrap().values()
                  .map(|value| value.read_string().unwrap().parse().unwrap()).collect::<Vec<u8>>().as_slice() {
                    [r, g, b] => (*r, *g, *b),
                    _ => (0, 0, 0),
                  };
                country_definitions.insert(tag.clone(), CountryDefinition {
                  tag: tag.clone(),
                  color: colors,
                });
              },
              Err(_) => {
                let hsv_type = value.read_array().unwrap().values().next().unwrap().read_string().unwrap();

                let color = match hsv_type.as_str() {
                  "hsv" => {
                    match value.read_array().unwrap().values().nth(1).unwrap().read_array().unwrap().values()
                      .map(|value| value.read_string().unwrap().parse().unwrap()).collect::<Vec<f64>>().as_slice() {
                        [hue, saturation, value] => ColorConverter::rgb_from_hsv_float(*hue, *saturation, *value),
                        _ => (0, 0, 0),
                      }
                  },
                  "hsv360" => {
                    match value.read_array().unwrap().values().nth(1).unwrap().read_array().unwrap().values()
                      .map(|value| value.read_string().unwrap().parse().unwrap()).collect::<Vec<i64>>().as_slice() {
                        [hue, saturation, value] => ColorConverter::rgb_from_hsv_degrees(*hue, *saturation, *value),
                        _ => (0, 0, 0),
                      }
                  },
                  _ => (0, 0, 0),
                };

                country_definitions.insert(tag.clone(), CountryDefinition {
                  tag: tag.clone(),
                  color,
                });
              }
            }
          }
        }
      }
    }

    country_definitions
  }
}
