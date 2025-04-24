use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PermissionsTable::Table)
                    .if_not_exists()
                    .col(pk_auto(PermissionsTable::Id))
                    .col(string(PermissionsTable::Name))
                    .col(string_null(PermissionsTable::Description))
                    .col(
                        timestamp_with_time_zone(PermissionsTable::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PermissionsTable::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PermissionsTable {
    #[sea_orm(iden = "permissions")]
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
}
