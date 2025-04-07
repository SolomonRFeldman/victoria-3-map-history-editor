pub use sea_orm_migration::prelude::*;

mod m20250329_151825_create_countries;
mod m20250330_004503_create_states;
mod m20250330_050749_create_pops;
mod m20250330_141936_create_buildings;
mod m20250330_151648_create_country_ownerships;
mod m20250330_151656_create_building_ownerships;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250329_151825_create_countries::Migration),
            Box::new(m20250330_004503_create_states::Migration),
            Box::new(m20250330_050749_create_pops::Migration),
            Box::new(m20250330_141936_create_buildings::Migration),
            Box::new(m20250330_151648_create_country_ownerships::Migration),
            Box::new(m20250330_151656_create_building_ownerships::Migration),
        ]
    }
}
