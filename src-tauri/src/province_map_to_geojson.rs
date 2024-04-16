use std::{collections::HashMap, path::PathBuf};

use geo::{LineString, MultiPolygon, Polygon};
use image::io::Reader as ImageReader;
use serde::Serialize;

#[derive(Clone)]
enum Direction {
  Up,
  Right,
  Down,
  Left,
}

struct Rotation {
  position: usize,
}

impl Rotation {
  fn new() -> Self {
    Self { position: 2 }
  }

  fn cycle(&self) -> [Direction; 4] {
    [
      Direction::Up,
      Direction::Right,
      Direction::Down,
      Direction::Left
    ]
  }

  fn cycle_forward(&mut self) {
    if self.position + 1 < self.cycle().len() {
      self.position += 1;
    } else {
      self.position = 0;
    }
  }

  fn cycle_backward(&mut self) {
    if self.position == 0 {
      self.position = self.cycle().len() - 1;
    } else {
      self.position -= 1;
    }
  }

  fn position_name(&self) -> Direction {
    self.cycle()[self.position].clone()
  }

  fn x_modifier(&self) -> i32 {
    match self.position_name() {
      Direction::Right => 1,
      Direction::Left => -1,
      _ => 0,
    }
  }

  fn y_modifier(&self) -> i32 {
    match self.position_name() {
      Direction::Up => 1,
      Direction::Down => -1,
      _ => 0,
    }
  }

  fn next_coord(&self, coord: (i32, i32)) -> (i32, i32) {
    (
      coord.0 + self.x_modifier(),
      coord.1 + self.y_modifier(),
    )
  }
}

// TO-DO: Code quality is in a bad state, should be refactored and broken up
// detect whether it intersects itself at a point where it looks like a T
pub fn border_to_geojson_coords(border_coords: Vec<(i32, i32)>) -> Vec<Vec<(f32, f32)>> {
  let border_coords: Vec<(i32, i32)> = border_coords.into_iter().map(|(x, y)| (x as i32, y as i32)).collect();
  let hash_coords: std::collections::HashSet<_> = border_coords.clone().into_iter().map(|(x, y)| (x as i32, y as i32)).collect();

  let origin_coord = border_coords[0];

  let mut geo_json_coordinate_array: Vec<Vec<(f32, f32)>> = vec![];
  parse_hash_set(hash_coords, origin_coord, &mut geo_json_coordinate_array).to_vec()
}

fn parse_hash_set(mut hash_coords: std::collections::HashSet<(i32, i32)>, start_point: (i32, i32), mut geo_json_coordinate_array: &mut Vec<Vec<(f32, f32)>>) -> &mut Vec<Vec<(f32, f32)>> {
  let mut rotation = Rotation::new();

  let origin_coord = start_point;
  let mut geo_trace = vec![origin_coord];

  let mut current_coord = origin_coord;

  loop {
    let next_coord = rotation.next_coord(current_coord);
    if hash_coords.contains(&next_coord) {
      geo_trace.push(next_coord);
      if next_coord == origin_coord {
        break;
      }
      current_coord = next_coord;
      continue;
    } else {
      rotation.cycle_forward();
    }

    let next_coord = rotation.next_coord(current_coord);
    if hash_coords.contains(&next_coord) {
      geo_trace.push(next_coord);
      if next_coord == origin_coord {
        break;
      }
      current_coord = next_coord;
      continue;
    } else {
      rotation.cycle_backward();
      rotation.cycle_backward();
    }
    
    let next_coord = rotation.next_coord(current_coord);
    if hash_coords.contains(&next_coord) {
      geo_trace.push(next_coord);
      if next_coord == origin_coord {
        break;
      }
      current_coord = next_coord;
      continue;
    } else {
      break;
    }
  }
  for coord in geo_trace.iter() {
    hash_coords.remove(coord);
  }

  if geo_trace.first() == geo_trace.last() && geo_trace.len() > 1 {
    let geo_json_coordinate = remove_unnecessary_coords(geo_trace);
  
    geo_json_coordinate_array.push(geo_json_coordinate.iter().map(|(x, y)| (*x as f32 / 2 as f32, *y as f32 / 2 as f32)).collect());
  }


  if hash_coords.len() > 0 {
    let next_start_point = hash_coords.iter().next().unwrap().clone();
    geo_json_coordinate_array = parse_hash_set(hash_coords, next_start_point, geo_json_coordinate_array);
  }

  geo_json_coordinate_array
}

fn remove_unnecessary_coords(geo_trace: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
  let mut new_geo_trace: Vec<Option<(i32, i32)>> = vec![Some(geo_trace[0])];

  for i in 1..geo_trace.len() - 1 {
    let coord = geo_trace[i];
    let backward_coord = geo_trace[i - 1];
    let forward_coord = geo_trace[i + 1];

    let backward_y_diff = (coord.0 - backward_coord.0).signum();
    let backward_x_diff = (coord.1 - backward_coord.1).signum();
    let forward_y_diff = (forward_coord.0 - coord.0).signum();
    let forward_x_diff = (forward_coord.1 - coord.1).signum();

    if (backward_y_diff, backward_x_diff) == (forward_y_diff, forward_x_diff) {
      new_geo_trace.push(None);
    } else {
      new_geo_trace.push(Some(coord));
    }
  }

  new_geo_trace.push(Some(*geo_trace.last().unwrap()));

  new_geo_trace.into_iter().filter_map(|x| x).collect()
}

pub fn province_map_to_geojson(provinces: PathBuf) -> HashMap<String, Vec<Vec<(f32, f32)>>> {
  let provinces = ImageReader::open(provinces).unwrap().decode().unwrap().into_rgb8();
  let mut province_borders: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
  let image_height = provinces.height() as i32;

  for (x, y, pixel) in provinces.enumerate_pixels() {
    let province_id = format!("x{:02X}{:02X}{:02X}", pixel[0], pixel[1], pixel[2]);

    // x641238 xE5C4FE xC7847D xA060C0 x5DEF58 x85F13B x656D90 x20E0C0

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

  let province_borders: HashMap<String, Vec<Vec<(f32,f32)>>> = province_borders.iter().map(|(hex_color, coords)| {
    let geo_json_coords = border_to_geojson_coords(coords.clone());

    (hex_color.clone(), geo_json_coords)
  }).collect();

  province_borders
}
