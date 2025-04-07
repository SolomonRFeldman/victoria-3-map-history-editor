use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Countries::Table)
                    .if_not_exists()
                    .col(pk_auto(Countries::Id))
                    .col(string(Countries::Tag))
                    .col(string(Countries::Color))
                    .col(string(Countries::Setup))
                    .col(string(Countries::Border))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-countries-id")
                    .table(Countries::Table)
                    .col(Countries::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Countries::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Countries {
    Table,
    Id,
    Tag,
    Color,
    Setup,
    Border,
}
