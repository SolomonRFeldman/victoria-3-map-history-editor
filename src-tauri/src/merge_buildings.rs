use crate::get_state_buildings::StateBuilding;

pub fn merge_state_buildings(
    buildings1: Vec<StateBuilding>,
    buildings2: Vec<StateBuilding>,
) -> Vec<StateBuilding> {
    let mut new_buildings = buildings1.clone();
    for building in buildings2 {
        let existing_building = new_buildings
            .iter_mut()
            .find(|new_building| new_building.name == building.name);
        match existing_building {
            Some(existing_building) => {
                if let Some(level) = existing_building.level {
                    existing_building.level = Some(level + building.level.unwrap());
                }
            }
            None => {
                new_buildings.push(building);
            }
        }
    }

    new_buildings
}
