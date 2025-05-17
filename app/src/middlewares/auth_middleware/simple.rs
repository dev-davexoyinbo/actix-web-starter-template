use actix_web::{
    HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::StatusCode,
    middleware::Next,
    web,
};
use app_errors::{AppError, UserError};
use chrono::Utc;
use entity::{
    auth_tokens, links::UserToPermissionsOfRole, permissions, roles,
    sea_orm_active_enums::TokenType, users,
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

use crate::{
    api::auth_module::{
        auth_extractors::{AuthExtractor, MaybeAuthExtractor},
        auth_models::AuthInfo,
    },
    persistence_state::PersistenceState,
};

pub async fn auth_middleware_global(
    persistence_state: web::Data<PersistenceState>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    if let Some(Ok(auth_header)) = req.headers().get("Authorization").map(|val| val.to_str()) {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            let db = &persistence_state.db;

            let auth_token = auth_tokens::Entity::find()
                .filter(
                    Condition::all()
                        .add(auth_tokens::Column::Token.eq(token))
                        .add(auth_tokens::Column::TokenType.eq(TokenType::AccessToken))
                        .add(
                            Condition::any()
                                .add(auth_tokens::Column::ExpiresAt.is_null())
                                .add(auth_tokens::Column::ExpiresAt.gt(chrono::Utc::now())),
                        ),
                )
                .find_also_related(users::Entity)
                .one(db)
                .await
                .map_err(AppError::DbErr)
                .map_err(Into::<UserError>::into)?;

            if let Some((auth_token, Some(user))) = auth_token {
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

                let permissions = permissions.into_iter().fold(Vec::new(), |mut acc, x| {
                    if acc.contains(&x) {
                        return acc;
                    }

                    acc.push(x);
                    acc
                });

                let auth_info = AuthInfo {
                    access_token: auth_token.token,
                    user_id: user.id,
                    email: user.email,
                    name: user.name,
                    status: user.status,
                    email_verified_at: user.email_verified_at.map(Into::into),
                    roles,
                    permissions,
                };

                req.extensions_mut().insert(auth_info);
            }
        }
    }

    next.call(req).await
} // end function auth_middleware_global

pub async fn require_auth_middleware(
    _auth_info: AuthExtractor,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    next.call(req).await
}

pub async fn guest_middleware(
    auth_info: MaybeAuthExtractor,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    if auth_info.is_some() {
        let err = UserError::from_message(
            "Resource only accessible to guests",
            StatusCode::UNAUTHORIZED,
        );
        return Err(err.into());
    }

    next.call(req).await
}

pub async fn require_email_verification(
    auth_info: AuthExtractor,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let auth_info = auth_info.into_inner();

    let Some(email_verified_at) = auth_info.email_verified_at else {
        let err = UserError::from_message("Email not verified", StatusCode::FORBIDDEN);
        return Err(err.into());
    };

    if email_verified_at > Utc::now() {
        let err =
            UserError::from_message("Email verification status in future", StatusCode::FORBIDDEN);

        tracing::error!(
            account_id = auth_info.user_id,
            "Email verification status in future"
        );

        return Err(err.into());
    }

    next.call(req).await
}
