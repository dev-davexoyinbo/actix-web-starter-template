use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250329_214625_create_roles_table::RoleTable,
    m20250329_214907_create_permissions_table::PermissionsTable,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RolePermissionTable::Table)
                    .if_not_exists()
                    .col(integer(RolePermissionTable::RoleId))
                    .col(integer(RolePermissionTable::PermissionId))
                    .primary_key(
                        Index::create()
                            .col(RolePermissionTable::RoleId)
                            .col(RolePermissionTable::PermissionId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RolePermissionTable::Table, RolePermissionTable::RoleId)
                            .to(RoleTable::Table, RoleTable::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                RolePermissionTable::Table,
                                RolePermissionTable::PermissionId,
                            )
                            .to(PermissionsTable::Table, PermissionsTable::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        timestamp_with_time_zone(RolePermissionTable::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RolePermissionTable::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum RolePermissionTable {
    #[sea_orm(iden = "role_permissions")]
    Table,
    RoleId,
    PermissionId,
    CreatedAt,
}
