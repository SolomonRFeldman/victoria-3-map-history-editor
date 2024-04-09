#[derive(Clone)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

struct Rotation {
  position: usize,
}

impl Rotation {
  fn new() -> Self {
      Self { position: 3 }
  }

  fn cycle(&self) -> [Direction; 8] {
    [
      Direction::Up,
      Direction::UpRight,
      Direction::Right,
      Direction::DownRight,
      Direction::Down,
      Direction::DownLeft,
      Direction::Left,
      Direction::UpLeft,
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
        Direction::UpRight => 1,
        Direction::DownRight => 1,
        Direction::Left => -1,
        Direction::UpLeft => -1,
        Direction::DownLeft => -1,
        _ => 0,
    }
  }

  fn y_modifier(&self) -> i32 {
      match self.position_name() {
          Direction::Up => 1,
          Direction::UpLeft => 1,
          Direction::UpRight => 1,
          Direction::Down => -1,
          Direction::DownLeft => -1,
          Direction::DownRight => -1,
          _ => 0,
      }
  }
}

// TO-DO: Code quality is in a bad state, should be refactored and broken up
pub fn border_to_geojson_coords(border_coords: Vec<(u32, u32)>) -> Vec<(i32, i32)> {
  let border_coords: Vec<(i32, i32)> = border_coords.into_iter().map(|(x, y)| (x as i32, y as i32)).collect();
  let mut rotation = Rotation::new();
  let hash_coords: std::collections::HashSet<_> = border_coords.clone().into_iter().map(|(x, y)| (x as i32, y as i32)).collect();

  let origin_coord = border_coords[0];
  let mut geo_trace = vec![origin_coord];

  let mut coord = None;

  let mut loop_count = 0;

  loop {
    if loop_count > 10000 {
      println!("Loop count exceeded 10000, breaking loop");
      break;
    }
    loop_count += 1;
    let current_coord = coord.unwrap_or(origin_coord);
    rotation.cycle_backward();
    rotation.cycle_backward();
    let mut found_coord = None;

    for _ in 0..8 {
        if found_coord.is_some() {
            break;
        }

        let key = (
            current_coord.0 + rotation.x_modifier(),
            current_coord.1 + rotation.y_modifier(),
        );

        if hash_coords.contains(&key) {
            found_coord = Some(key);
            coord = found_coord;
            geo_trace.push(key);
        } else {
            rotation.cycle_forward();
        }
    }

    if coord == Some(origin_coord) {
        break;
    }
  }

  remove_unnecessary_coords(geo_trace)
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
