use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RoleTable::Table)
                    .if_not_exists()
                    .col(pk_auto(RoleTable::Id))
                    .col(string(RoleTable::Name))
                    .col(string_null(RoleTable::Description))
                    .col(
                        timestamp_with_time_zone(RoleTable::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RoleTable::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum RoleTable {
    #[sea_orm(iden = "roles")]
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
}
