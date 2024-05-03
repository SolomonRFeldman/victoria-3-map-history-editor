use crate::{country::Country, get_states::State};
use image::{io::Reader as ImageReader, Rgb};
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub type Coords = Vec<Vec<(f32, f32)>>;

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
            Direction::Left,
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
        (coord.0 + self.x_modifier(), coord.1 + self.y_modifier())
    }
}

// TO-DO: Code quality is in a bad state, should be refactored and broken up
// detect whether it intersects itself at a point where it looks like a T
pub fn border_to_geojson_coords(border_coords: Vec<(i32, i32)>) -> Coords {
    let border_coords: Vec<(i32, i32)> = border_coords.into_iter().collect();
    let hash_coords: std::collections::HashSet<_> = border_coords.clone().into_iter().collect();

    let origin_coord = border_coords[0];

    let mut geo_json_coordinate_array: Coords = vec![];
    parse_hash_set(hash_coords, origin_coord, &mut geo_json_coordinate_array).to_vec()
}

fn parse_hash_set(
    mut hash_coords: std::collections::HashSet<(i32, i32)>,
    start_point: (i32, i32),
    mut geo_json_coordinate_array: &mut Coords,
) -> &mut Coords {
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

        geo_json_coordinate_array.push(
            geo_json_coordinate
                .iter()
                .map(|(x, y)| (*x as f32 / 2_f32, *y as f32 / 2_f32))
                .collect(),
        );
    }

    if !hash_coords.is_empty() {
        let next_start_point = *hash_coords.iter().next().unwrap();
        geo_json_coordinate_array =
            parse_hash_set(hash_coords, next_start_point, geo_json_coordinate_array);
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

    new_geo_trace.into_iter().flatten().collect()
}

pub fn province_map_to_geojson(provinces: PathBuf) -> HashMap<String, Coords> {
    let provinces = ImageReader::open(provinces)
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb8();
    let mut province_borders: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
    let image_height = provinces.height() as i32;

    for (x, y, pixel) in provinces.enumerate_pixels() {
        let province_id = format!("x{:02X}{:02X}{:02X}", pixel[0], pixel[1], pixel[2]);

        let x = x as i32;
        let y = y as i32;

        let top: (i32, i32, i32, i32, i32, i32, i32, i32) = (x, y - 1, 0, 0, 2, 0, 1, 0);
        let left: (i32, i32, i32, i32, i32, i32, i32, i32) = (x - 1, y, 0, 0, 0, 2, 0, 1);
        let right: (i32, i32, i32, i32, i32, i32, i32, i32) = (x + 1, y, 2, 2, 2, 0, 2, 1);
        let bottom: (i32, i32, i32, i32, i32, i32, i32, i32) = (x, y + 1, 0, 2, 2, 2, 1, 2);

        let neighbors = [bottom, top, left, right];

        neighbors.iter().for_each(|&neighbor| {
            if neighbor.0 < 0
                || neighbor.1 < 0
                || neighbor.0 >= provinces.width() as i32
                || neighbor.1 >= provinces.height() as i32
                || provinces.get_pixel(neighbor.0 as u32, neighbor.1 as u32) != pixel
            {
                province_borders
                    .entry(province_id.clone())
                    .or_default()
                    .push((
                        (x * 2) + neighbor.2,
                        (image_height * 2) - ((y * 2) + neighbor.3),
                    ));
                province_borders
                    .entry(province_id.clone())
                    .or_default()
                    .push((
                        (x * 2) + neighbor.4,
                        (image_height * 2) - ((y * 2) + neighbor.5),
                    ));
                province_borders
                    .entry(province_id.clone())
                    .or_default()
                    .push((
                        (x * 2) + neighbor.6,
                        (image_height * 2) - ((y * 2) + neighbor.7),
                    ));
            }
        });
    }

    province_borders
        .iter()
        .map(|(hex_color, coords)| {
            let geo_json_coords = border_to_geojson_coords(coords.clone());

            (hex_color.clone(), geo_json_coords)
        })
        .collect()
}

pub fn state_map_to_geojson(
    province_map: PathBuf,
    state_map: PathBuf,
    states: Vec<State>,
) -> HashMap<String, Coords> {
    if fs::metadata(&state_map).is_err() {
        let mut color_map = HashMap::<Rgb<u8>, Rgb<u8>>::new();
        states.iter().for_each(|state| {
            state.sub_states.iter().for_each(|sub_state| {
                if sub_state.provinces.is_empty() {
                    println!("No valid provinces for state: {:?}", state.name);
                    return;
                }
                let first_province = sub_state.provinces[0].trim_matches('"');
                let red: String = first_province.chars().skip(1).take(2).collect::<String>();
                let green: String = first_province.chars().skip(3).take(2).collect::<String>();
                let blue: String = first_province.chars().skip(5).take(2).collect::<String>();

                let color_to_turn = Rgb([
                    u8::from_str_radix(&red, 16).unwrap(),
                    u8::from_str_radix(&green, 16).unwrap(),
                    u8::from_str_radix(&blue, 16).unwrap(),
                ]);
                sub_state.provinces.iter().for_each(|province| {
                    let red = province.chars().skip(1).take(2).collect::<String>();
                    let green = province.chars().skip(3).take(2).collect::<String>();
                    let blue = province.chars().skip(5).take(2).collect::<String>();

                    let color = Rgb([
                        u8::from_str_radix(&red, 16).unwrap(),
                        u8::from_str_radix(&green, 16).unwrap(),
                        u8::from_str_radix(&blue, 16).unwrap(),
                    ]);
                    color_map.insert(color, color_to_turn);
                })
            });
        });

        let mut provinces = ImageReader::open(province_map)
            .unwrap()
            .decode()
            .unwrap()
            .into_rgb8();

        provinces.enumerate_pixels_mut().for_each(|(_, _, pixel)| {
            let color = color_map.get(pixel).unwrap_or(&Rgb([0, 0, 0]));
            *pixel = *color;
        });
        provinces.save(&state_map).unwrap();
    } else {
        println!("State map already in cache");
    }

    let state_borders = province_map_to_geojson(state_map);

    let mut state_map: HashMap<String, Coords> = HashMap::new();
    states.iter().for_each(|state| {
        state.sub_states.iter().for_each(|sub_state| {
            state_map.insert(
                format!("{}:{}", sub_state.owner.clone(), state.name.clone()),
                state_borders.get(&sub_state.provinces[0]).unwrap().clone(),
            );
        });
    });
    state_map
}

pub fn country_map_to_geojson(
    state_map: PathBuf,
    country_map: PathBuf,
    countries: Vec<Country>,
) -> Vec<Country> {
    if fs::metadata(&country_map).is_err() {
        let mut color_map = HashMap::<Rgb<u8>, Rgb<u8>>::new();
        countries.iter().for_each(|country| {
            let color_to_turn = Rgb([
                country.name.chars().next().unwrap() as u8,
                country.name.chars().nth(1).unwrap() as u8,
                country.name.chars().nth(2).unwrap() as u8,
            ]);

            country.states.iter().for_each(|state| {
                let state_color_id = &state.provinces[0];
                let r = u8::from_str_radix(&state_color_id[1..3], 16).ok().unwrap();
                let g = u8::from_str_radix(&state_color_id[3..5], 16).ok().unwrap();
                let b = u8::from_str_radix(&state_color_id[5..7], 16).ok().unwrap();

                color_map.insert(Rgb([r, g, b]), color_to_turn);
            });
        });

        let mut state_map_image = ImageReader::open(state_map)
            .unwrap()
            .decode()
            .unwrap()
            .into_rgb8();

        state_map_image
            .enumerate_pixels_mut()
            .for_each(|(_, _, pixel)| {
                let color = color_map.get(pixel).unwrap_or(&Rgb([0, 0, 0]));
                *pixel = *color;
            });
        state_map_image.save(&country_map).unwrap();
    } else {
        println!("Country map already in cache");
    }

    let country_borders = province_map_to_geojson(country_map);

    countries
        .iter()
        .map(|country| {
            let color = format!(
                "{:02x}{:02x}{:02x}",
                country.name.chars().next().unwrap() as u8,
                country.name.chars().nth(1).unwrap() as u8,
                country.name.chars().nth(2).unwrap() as u8
            )
            .to_uppercase();
            let country_coords = country_borders.get(&format!("x{}", color));

            match country_coords {
                Some(geometries) => Country {
                    name: country.name.clone(),
                    color: country.color,
                    states: country.states.clone(),
                    coordinates: geometries.to_vec(),
                    setup: country.setup.clone(),
                },
                None => {
                    println!(
                        "No geometries for country: {:?}, {:?}",
                        country.name, country.color
                    );
                    Country {
                        name: country.name.clone(),
                        color: country.color,
                        states: country.states.clone(),
                        coordinates: vec![],
                        setup: country.setup.clone(),
                    }
                }
            }
        })
        .collect::<Vec<Country>>()
}
