use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250330_004503_create_states::States;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Pops::Table)
                    .if_not_exists()
                    .col(pk_auto(Pops::Id))
                    .col(integer(Pops::StateId))
                    .col(string(Pops::Culture))
                    .col(ColumnDef::new(Pops::Religion).string())
                    .col(integer(Pops::Size))
                    .col(ColumnDef::new(Pops::PopType).string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-pops-state-id")
                            .from(Pops::Table, Pops::StateId)
                            .to(States::Table, States::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-pops-id")
                    .table(Pops::Table)
                    .col(Pops::Id)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-pops-state-id")
                    .table(Pops::Table)
                    .col(Pops::StateId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Pops::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Pops {
    Table,
    Id,
    StateId,
    Culture,
    Religion,
    Size,
    PopType,
}
