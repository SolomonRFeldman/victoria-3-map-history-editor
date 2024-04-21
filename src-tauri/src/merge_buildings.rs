use crate::get_state_buildings::Building;

pub fn merge_buildings(buildings1: Vec<Building>, buildings2: Vec<Building>) -> Vec<Building> {
  let mut new_buildings = buildings1.clone();
  for building in buildings2 {
    let existing_building = new_buildings.iter_mut().find(|new_building| new_building.name == building.name);
    match existing_building {
      Some(existing_building) => {
        match existing_building.level {
          Some(level) => {
            existing_building.level = Some(level + building.level.unwrap());
          },
          None => {},
        }
      },
      None => {
        new_buildings.push(building);
      },
    }
  }

  new_buildings
}
