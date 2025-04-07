use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250329_151825_create_countries::Countries, m20250330_141936_create_buildings::Buildings,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CountryOwnerships::Table)
                    .if_not_exists()
                    .col(pk_auto(CountryOwnerships::Id))
                    .col(integer(CountryOwnerships::BuildingId))
                    .col(integer(CountryOwnerships::CountryId))
                    .col(integer(CountryOwnerships::Levels))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-country_ownerships-building-id")
                            .from(CountryOwnerships::Table, CountryOwnerships::BuildingId)
                            .to(Buildings::Table, Buildings::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-country_ownerships-country-id")
                            .from(CountryOwnerships::Table, CountryOwnerships::CountryId)
                            .to(Countries::Table, Countries::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-country_ownerships-id")
                    .table(CountryOwnerships::Table)
                    .col(CountryOwnerships::Id)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-country_ownerships-building-id")
                    .table(CountryOwnerships::Table)
                    .col(CountryOwnerships::BuildingId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-country_ownerships-country-id")
                    .table(CountryOwnerships::Table)
                    .col(CountryOwnerships::CountryId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CountryOwnerships::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CountryOwnerships {
    Table,
    Id,
    BuildingId,
    CountryId,
    Levels,
}
