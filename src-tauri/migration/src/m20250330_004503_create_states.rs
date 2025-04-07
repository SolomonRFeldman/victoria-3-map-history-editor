use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250329_151825_create_countries::Countries;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(States::Table)
                    .if_not_exists()
                    .col(pk_auto(States::Id))
                    .col(integer(States::CountryId))
                    .col(string(States::Name))
                    .col(string(States::Provinces))
                    .col(string(States::Border))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-states-country-id")
                            .from(States::Table, States::CountryId)
                            .to(Countries::Table, Countries::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-states-id")
                    .table(States::Table)
                    .col(States::Id)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-states-country-id")
                    .table(States::Table)
                    .col(States::CountryId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(States::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum States {
    Table,
    Id,
    CountryId,
    Name,
    Provinces,
    Border,
}
