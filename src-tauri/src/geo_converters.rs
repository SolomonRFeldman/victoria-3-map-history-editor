use geo::{MultiPolygon, Polygon, LineString};

pub fn multi_poly_to_vec(multi_poly: MultiPolygon<f32>) -> Vec<Vec<(f32, f32)>> {
  let mut exterior_coords = multi_poly.clone().into_iter().map(|poly| { line_string_to_vec(poly.exterior().clone()) }).collect::<Vec<Vec<(f32, f32)>>>();
  let interior_coords = multi_poly.clone().into_iter().map(|poly| { poly.interiors().iter().map(|line_string| { line_string_to_vec(line_string.clone()) }).collect::<Vec<Vec<(f32, f32)>>>() }).collect::<Vec<Vec<Vec<(f32, f32)>>>>();
  exterior_coords.extend(interior_coords.into_iter().flatten());
  exterior_coords
}

pub fn line_string_to_vec(line_string: LineString<f32>) -> Vec<(f32, f32)> {
  line_string.into_iter().map(|point| { (point.x, point.y) }).collect()
}

pub fn vec_to_multi_poly(coords: Vec<Vec<(f32, f32)>>) -> MultiPolygon<f32> {
  coords.into_iter().map(|coords| { Polygon::new(LineString::from(coords), vec![]) }).collect()
}
