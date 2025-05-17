use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250324_002438_create_users_table::UserTable, m20250329_214625_create_roles_table::RoleTable,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRole::Table)
                    .if_not_exists()
                    .col(big_integer(UserRole::UserId))
                    .col(integer(UserRole::RoleId))
                    .primary_key(Index::create().col(UserRole::UserId).col(UserRole::RoleId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserRole::Table, UserRole::RoleId)
                            .to(RoleTable::Table, RoleTable::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserRole::Table, UserRole::UserId)
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
            .drop_table(Table::drop().table(UserRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserRole {
    #[sea_orm(iden = "user_role")]
    Table,
    UserId,
    RoleId,
}
