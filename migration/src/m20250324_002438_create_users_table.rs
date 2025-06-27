use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
    sea_orm::{EnumIter, Iterable},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(UserStatus)
                    .values(UserStatusVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserTable::Table)
                    .if_not_exists()
                    .col(big_integer(UserTable::Id).auto_increment().primary_key())
                    .col(string(UserTable::Name))
                    .col(string(UserTable::Email).unique_key())
                    .col(
                        enumeration(UserTable::Status, UserStatus, UserStatusVariants::iter())
                            .default("active"),
                    )
                    .col(string(UserTable::Password))
                    .col(timestamp_with_time_zone_null(UserTable::EmailVerifiedAt))
                    .col(
                        timestamp_with_time_zone(UserTable::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(UserTable::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserTable::Table).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(UserStatus)
                    .restrict()
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserTable {
    #[sea_orm(iden = "users")]
    Table,
    Id,
    Name,
    Email,
    Status,
    Password,
    EmailVerifiedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub struct UserStatus;

#[derive(Iden, EnumIter)]
pub enum UserStatusVariants {
    #[iden = "active"]
    Active,
    #[iden = "inactive"]
    Inactive,
    #[iden = "banned"]
    Banned,
}
