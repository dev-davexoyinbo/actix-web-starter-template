use actix_web::http::StatusCode;
use app_errors::{AppError, Error, UserError};
use chrono::{DateTime, Utc};
use common::helpers::generate_random_string;
use entity::sea_orm_active_enums::TokenType;
use entity::{auth_tokens, users};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, TransactionTrait,
};
use tracing::instrument;

use crate::{globals, persistence_state};

use super::auth_dtos::{
    LoginRequestDto, LoginResponseDto, RegisterRequestDto, RegisterRequestResponseDto,
};

pub struct AuthService;

impl AuthService {
    #[instrument(skip_all)]
    pub async fn register(dto: RegisterRequestDto) -> Result<RegisterRequestResponseDto, Error> {
        let conn = &persistence_state::get()?.db;

        let user = conn
            .transaction::<_, users::Model, Error>(|tx| {
                Box::pin(async move {
                    let email = dto.email.to_lowercase();

                    let exists: Option<i64> = users::Entity::find()
                        .filter(users::Column::Email.eq(&email))
                        .select_only()
                        .column(users::Column::Id)
                        .into_tuple()
                        .one(tx)
                        .await
                        .map_err(AppError::DbErr)?;

                    if exists.is_some() {
                        let err = UserError::from_message(
                            "Email already exists",
                            StatusCode::BAD_REQUEST,
                        );
                        return Err(err.into());
                    }

                    let password = globals::password::get()?
                        .hash_password(&dto.password)
                        .map_err(Into::<AppError>::into)?;

                    let account = users::ActiveModel {
                        name: Set(dto.name),
                        password: Set(password),
                        email: Set(email),
                        ..Default::default()
                    }
                    .insert(tx)
                    .await
                    .map_err(AppError::DbErr)?;

                    Ok(account)
                })
            })
            .await?;

        tracing::info!(user.id, "User registered successfully",);

        Ok(RegisterRequestResponseDto { id: user.id })
    } // end function register

    #[instrument(skip_all)]
    pub async fn login(dto: LoginRequestDto) -> Result<LoginResponseDto, Error> {
        let conn = &persistence_state::get()?.db;

        let user = users::Entity::find()
            .filter(users::Column::Email.eq(dto.email.to_lowercase()))
            .one(conn)
            .await
            .map_err(AppError::DbErr)?;

        let Some(account) = user else {
            let err = UserError::from_message("Invalid email or password", StatusCode::NOT_FOUND);

            return Err(err.into());
        };

        if !globals::password::get()?
            .verify_hashed_password(&account.password, &dto.password)
            .map_err(|err| {
                UserError::from_message(
                    &format!("Failed to verify password: {}", err),
                    StatusCode::BAD_REQUEST,
                )
            })?
        {
            let err = UserError::from_message("Invalid email or password", StatusCode::BAD_REQUEST);

            return Err(err.into());
        }

        let auth_token =
            Self::create_uniq_auth_token(32, account.id, TokenType::AccessToken, None, None)
                .await?;

        tracing::info!(account.id, "User logged in successfully",);

        Ok(LoginResponseDto {
            id: account.id,
            token: auth_token.token,
        })
    } // end function login

    async fn create_uniq_auth_token(
        length: u8,
        user_id: i64,
        token_type: TokenType,
        meta: Option<serde_json::Value>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<auth_tokens::Model, Error> {
        let conn = &persistence_state::get()?.db;

        let auth_token = conn
            .transaction::<_, auth_tokens::Model, Error>(|tx| {
                Box::pin(async move {
                    let mut count = 0;

                    let token: String = loop {
                        if count >= 5 {
                            let err = UserError::from_message(
                                "Failed to generate a unique token",
                                StatusCode::INTERNAL_SERVER_ERROR,
                            );
                            return Err(err.into());
                        }

                        count += 1;

                        let token = generate_random_string(length);

                        let exists = auth_tokens::Entity::find()
                            .filter(auth_tokens::Column::Token.eq(&token))
                            .select_only()
                            .column(auth_tokens::Column::Id)
                            .one(tx)
                            .await
                            .map_err(AppError::DbErr)?;

                        if exists.is_some() {
                            continue;
                        }

                        break token;
                    };

                    let auth_token = auth_tokens::ActiveModel {
                        user_id: Set(user_id),
                        token: Set(token),
                        token_type: Set(token_type),
                        expires_at: Set(expires_at.map(Into::into)),
                        meta: Set(meta),
                        ..Default::default()
                    }
                    .insert(tx)
                    .await
                    .map_err(AppError::DbErr)?;

                    Ok(auth_token)
                })
            })
            .await?;

        Ok(auth_token)
    } // end function
} // end impl for AuthService
