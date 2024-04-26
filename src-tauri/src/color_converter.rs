use palette::{convert::FromColorUnclamped, rgb as PaletteRgb, Hsv };

pub struct ColorConverter;

impl ColorConverter {
  pub fn rgb_from_hsv_degrees(hue: i64, saturation: i64, brightness_value: i64) -> (u8, u8, u8) {
    let hsv = Hsv::new(hue as f64, saturation as f64 / 100.0, brightness_value as f64 / 100.0); // Hue in degrees, Saturation and Value from 0.0 to 1.0
    let rgb: PaletteRgb::Rgb<_, u8> = PaletteRgb::Rgb::from_color_unclamped(hsv).into();

    (rgb.red, rgb.green, rgb.blue)
  }

  pub fn rgb_from_hsv_float(hue: f64, saturation: f64, brightness_value: f64) -> (u8, u8, u8) {
    let hsv = Hsv::new(hue, saturation, brightness_value); // Hue in degrees, Saturation and Value from 0.0 to 1.0
    let rgb: PaletteRgb::Rgb<_, u8> = PaletteRgb::Rgb::from_color_unclamped(hsv).into();

    (rgb.red, rgb.green, rgb.blue)
  }
}
