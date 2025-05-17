use actix_web::http::StatusCode;
use app_errors::{AppError, Error, UserError};
use chrono::{DateTime, Utc};
use common::helpers::{generate_random_numeric_string, generate_random_string};
use entity::links::UserToPermissionsOfRole;
use entity::sea_orm_active_enums::TokenType;
use entity::{auth_tokens, permissions, roles, users};
use messaging::{AppMessageTopic, AppMessageWrapper, VerifyEmailTemplate};
use sea_orm::ActiveValue::{Set, Unchanged};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseTransaction, EntityTrait, QueryFilter,
    QuerySelect, TransactionTrait,
};
use tracing::instrument;

use crate::{app_config, globals, persistence_state};

use super::auth_dtos::{
    LoginRequestDto, LoginResponseDto, RegisterRequestDto, RegisterRequestResponseDto,
    ResendVerificationEmailRequestDto, UserMeResponseDto, VerifyEmailRequestDto,
};

struct CreateUniqTokenArguments {
    length: u8,
    user_id: i64,
    token_type: TokenType,
    meta: Option<serde_json::Value>,
    expires_at: Option<DateTime<Utc>>,
    is_numeric: bool,
}

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

                    let user = users::ActiveModel {
                        name: Set(dto.name),
                        password: Set(password),
                        email: Set(email),
                        ..Default::default()
                    }
                    .insert(tx)
                    .await
                    .map_err(AppError::DbErr)?;

                    Self::send_verification_email(tx, user.id, &user.name, &user.email).await?;

                    Ok(user)
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

        let Some(user) = user else {
            let err = UserError::from_message("Invalid email or password", StatusCode::NOT_FOUND);

            return Err(err.into());
        };

        if !globals::password::get()?
            .verify_hashed_password(&user.password, &dto.password)
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

        let auth_token = Self::create_uniq_auth_token(
            CreateUniqTokenArguments {
                length: 32,
                user_id: user.id,
                token_type: TokenType::AccessToken,
                meta: None,
                expires_at: None,
                is_numeric: false,
            },
            None,
        )
        .await?;

        tracing::info!(user.id, "User logged in successfully",);

        Ok(LoginResponseDto {
            id: user.id,
            token: auth_token.token,
        })
    } // end function login

    #[instrument(skip_all)]
    pub async fn verify_email(
        VerifyEmailRequestDto { token, email }: VerifyEmailRequestDto,
    ) -> Result<(), Error> {
        let conn = &persistence_state::get()?.db;

        let account = users::Entity::find()
            .filter(users::Column::Email.eq(email.to_lowercase()))
            .one(conn)
            .await
            .map_err(AppError::DbErr)?;

        let Some(account) = account else {
            let err = UserError::from_message("User not found", StatusCode::NOT_FOUND);
            return Err(err.into());
        };

        let auth_token = auth_tokens::Entity::find()
            .filter(
                Condition::all()
                    .add(auth_tokens::Column::UserId.eq(account.id))
                    .add(auth_tokens::Column::Token.eq(token))
                    .add(
                        Condition::any()
                            .add(auth_tokens::Column::ExpiresAt.gt(Utc::now()))
                            .add(auth_tokens::Column::ExpiresAt.is_null()),
                    )
                    .add(auth_tokens::Column::TokenType.eq(TokenType::EmailOtp)),
            )
            .one(conn)
            .await
            .map_err(AppError::DbErr)?;

        if auth_token.is_none() {
            let err = UserError::from_message("Invalid token", StatusCode::BAD_REQUEST);
            return Err(err.into());
        }

        users::ActiveModel {
            id: Unchanged(account.id),
            email_verified_at: Set(Some(Utc::now().into())),
            ..Default::default()
        }
        .update(conn)
        .await
        .map_err(AppError::DbErr)?;

        Ok(())
    } // end function verify_email

    #[instrument(skip_all)]
    async fn create_uniq_auth_token(
        args: CreateUniqTokenArguments,
        conn: Option<&DatabaseTransaction>,
    ) -> Result<auth_tokens::Model, Error> {
        async fn helper(
            CreateUniqTokenArguments {
                length,
                user_id,
                token_type,
                meta,
                expires_at,
                is_numeric,
            }: CreateUniqTokenArguments,
            conn: &DatabaseTransaction,
        ) -> Result<auth_tokens::Model, Error> {
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

                            let token = match is_numeric {
                                true => generate_random_numeric_string(length),
                                false => generate_random_string(length),
                            };

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
        }

        match conn {
            Some(conn) => helper(args, conn).await,
            None => {
                let db = &persistence_state::get()?.db;

                let auth_token = db
                    .transaction::<_, auth_tokens::Model, Error>(|tx| {
                        Box::pin(async move {
                            let auth_token = helper(args, tx).await?;

                            Ok(auth_token)
                        })
                    })
                    .await?;

                Ok(auth_token)
            }
        }
    } // end function

    #[instrument(skip_all)]
    pub async fn me(user_id: i64) -> Result<UserMeResponseDto, Error> {
        let persistence_state = persistence_state::get()?;
        let db = &persistence_state.db;
        let user = users::Entity::find_by_id(user_id)
            .one(db)
            .await
            .map_err(AppError::DbErr)?;

        let Some(user) = user else {
            let err = UserError::from_message("Account not found", StatusCode::NOT_FOUND);
            return Err(err.into());
        };

        let roles: Vec<String> = users::Entity::find()
            .filter(users::Column::Id.eq(user.id))
            .find_with_related(roles::Entity)
            .all(db)
            .await
            .map_err(AppError::DbErr)
            .map_err(Into::<UserError>::into)?
            .into_iter()
            .flat_map(|role_user| role_user.1)
            .map(|role| role.name)
            .collect();

        let permissions_of_roles: Vec<String> = users::Entity::find()
            .filter(users::Column::Id.eq(user.id))
            .find_with_linked(UserToPermissionsOfRole)
            .all(db)
            .await
            .map_err(AppError::DbErr)
            .map_err(Into::<UserError>::into)?
            .into_iter()
            .flat_map(|p| p.1)
            .map(|p| p.name)
            .collect();

        let permissions_of_users: Vec<String> = users::Entity::find()
            .filter(users::Column::Id.eq(user.id))
            .find_with_related(permissions::Entity)
            .all(db)
            .await
            .map_err(AppError::DbErr)
            .map_err(Into::<UserError>::into)?
            .into_iter()
            .flat_map(|p| p.1)
            .map(|p| p.name)
            .collect();

        let mut permissions = permissions_of_roles;
        permissions.extend(permissions_of_users);

        Ok(UserMeResponseDto {
            id: user.id,
            name: user.name,
            email: user.email,
            status: user.status,
            email_verified_at: user.email_verified_at.map(Into::into),
            roles,
            permissions,
            created_at: user.created_at.into(),
            updated_at: user.updated_at.into(),
        })
    }

    #[instrument(skip_all)]
    pub async fn resend_verification_email(
        dto: ResendVerificationEmailRequestDto,
    ) -> Result<(), Error> {
        let db = &persistence_state::get()?.db;

        db.transaction::<_, (), Error>(|tx| {
            Box::pin(async move {
                let user = users::Entity::find()
                    .filter(users::Column::Email.eq(dto.email.to_lowercase()))
                    .one(tx)
                    .await
                    .map_err(AppError::DbErr)?;

                let Some(user) = user else {
                    let err = UserError::from_message("Account not found", StatusCode::NOT_FOUND);
                    return Err(err.into());
                };

                if user.email_verified_at.is_some() {
                    let err =
                        UserError::from_message("Email already verified", StatusCode::BAD_REQUEST);
                    return Err(err.into());
                }

                Self::send_verification_email(tx, user.id, &user.name, &user.email).await?;

                Ok(())
            })
        })
        .await?;
        Ok(())
    } // end function resend_verification_email

    async fn send_verification_email(
        tx: &DatabaseTransaction,
        user_id: i64,
        user_name: &str,
        user_email: &str,
    ) -> Result<(), Error> {
        let messaging_client = globals::messaging::get()?;
        let app_config = app_config::get()?;

        let otp = AuthService::create_uniq_auth_token(
            CreateUniqTokenArguments {
                length: 6,
                user_id,
                token_type: TokenType::EmailOtp,
                meta: None,
                expires_at: Some(Utc::now() + chrono::Duration::minutes(30)),
                is_numeric: true,
            },
            Some(tx),
        )
        .await?;

        let app_message_wrapper = AppMessageWrapper {
            key: None,
            topic: AppMessageTopic::GeneralEmail,
            message: messaging::EmailMessage {
                from: format!(
                    "{} <{}>",
                    app_config.app.name, app_config.messaging.default_email_from
                ),
                to: format!("{} <{}>", user_name, user_email),
                reply_to: None,
                subject: "Verify email".to_string(),
                template: VerifyEmailTemplate {
                    name: user_name.to_string(),
                    otp: otp.token,
                }
                .into(),
            }
            .into(),
        };

        messaging_client
            .read()
            .await
            .send_message(app_message_wrapper)
            .await
            .map_err(AppError::MessagingError)?;
        Ok(())
    } //end method send_verification_email
} // end impl for AuthService
