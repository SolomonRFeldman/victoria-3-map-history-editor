use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20250330_004503_create_states::States, m20250330_141936_create_buildings::Buildings};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BuildingOwnerships::Table)
                    .if_not_exists()
                    .col(pk_auto(BuildingOwnerships::Id))
                    .col(integer(BuildingOwnerships::BuildingId))
                    .col(integer(BuildingOwnerships::StateId))
                    .col(string(BuildingOwnerships::OwnerType))
                    .col(integer(BuildingOwnerships::Levels))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-building_ownerships-building-id")
                            .from(BuildingOwnerships::Table, BuildingOwnerships::BuildingId)
                            .to(Buildings::Table, Buildings::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-building_ownerships-state-id")
                            .from(BuildingOwnerships::Table, BuildingOwnerships::StateId)
                            .to(States::Table, States::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-building_ownerships-id")
                    .table(BuildingOwnerships::Table)
                    .col(BuildingOwnerships::Id)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-building_ownerships-building-id")
                    .table(BuildingOwnerships::Table)
                    .col(BuildingOwnerships::BuildingId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-building_ownerships-state-id")
                    .table(BuildingOwnerships::Table)
                    .col(BuildingOwnerships::StateId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BuildingOwnerships::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BuildingOwnerships {
    Table,
    Id,
    BuildingId,
    StateId,
    OwnerType,
    Levels,
}
