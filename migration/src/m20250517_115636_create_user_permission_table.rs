use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250324_002438_create_users_table::UserTable,
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
                    .table(UserPermission::Table)
                    .if_not_exists()
                    .col(integer(UserPermission::UserId))
                    .col(integer(UserPermission::PermissionId))
                    .primary_key(
                        Index::create()
                            .col(UserPermission::UserId)
                            .col(UserPermission::PermissionId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserPermission::Table, UserPermission::PermissionId)
                            .to(PermissionsTable::Table, PermissionsTable::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserPermission::Table, UserPermission::UserId)
                            .to(UserTable::Table, UserTable::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserPermission::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserPermission {
    #[sea_orm(iden = "user_permission")]
    Table,
    UserId,
    PermissionId,
}
