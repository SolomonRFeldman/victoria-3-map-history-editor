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
                    .table(Buildings::Table)
                    .if_not_exists()
                    .col(pk_auto(Buildings::Id))
                    .col(integer(Buildings::StateId))
                    .col(string(Buildings::Name))
                    .col(ColumnDef::new(Buildings::Level).integer())
                    .col(ColumnDef::new(Buildings::Reserves).integer())
                    .col(ColumnDef::new(Buildings::ActivateProductionMethods).string())
                    .col(ColumnDef::new(Buildings::Condition).string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-buildings-state-id")
                            .from(Buildings::Table, Buildings::StateId)
                            .to(States::Table, States::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-buildings-id")
                    .table(Buildings::Table)
                    .col(Buildings::Id)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-buildings-state-id")
                    .table(Buildings::Table)
                    .col(Buildings::StateId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Buildings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Buildings {
    Table,
    Id,
    StateId,
    Name,
    Level,
    Reserves,
    ActivateProductionMethods,
    Condition,
}
