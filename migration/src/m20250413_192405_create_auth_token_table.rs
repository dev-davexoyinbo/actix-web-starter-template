use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
    sea_orm::{EnumIter, Iterable},
};

use crate::m20250324_002438_create_users_table::UserTable;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(TokenType)
                    .values(TokenTypeVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AuthToken::Table)
                    .if_not_exists()
                    .col(pk_auto(AuthToken::Id))
                    .col(big_integer(AuthToken::UserId))
                    .col(string(AuthToken::Token).unique_key())
                    .col(enumeration(
                        AuthToken::TokenType,
                        TokenType,
                        TokenTypeVariants::iter(),
                    ))
                    .col(json_null(AuthToken::Meta)) // Nullable JSON field for metadata
                    .col(timestamp_with_time_zone_null(AuthToken::ExpiresAt))
                    .col(
                        timestamp_with_time_zone(AuthToken::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AuthToken::Table, AuthToken::UserId)
                            .to(UserTable::Table, UserTable::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuthToken::Table).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(TokenType)
                    .restrict()
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum AuthToken {
    #[sea_orm(iden = "auth_tokens")]
    Table,
    Id,
    UserId,
    Token,
    TokenType, // Enum column
    Meta,
    ExpiresAt,
    CreatedAt,
}

#[derive(DeriveIden)]
pub struct TokenType;

// Define the TokenType enum
#[derive(Iden, EnumIter)]
enum TokenTypeVariants {
    #[iden = "access_token"]
    AccessToken,
    #[iden = "email_otp"]
    EmailOtp,
    #[iden = "password_otp"]
    PasswordOtp,
}
