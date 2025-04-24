use actix_web::{
    HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::StatusCode,
    middleware::Next,
    web,
};
use app_errors::{AppError, UserError};
use entity::{auth_tokens, sea_orm_active_enums::TokenType, users};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

use crate::{api::auth_module::auth_models::AuthInfo, persistence_state::PersistenceState};

use super::auth_extractors::{AuthExtractor, MaybeAuthExtractor};

pub async fn auth_middleware_global(
    persistence_state: web::Data<PersistenceState>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    tracing::info!(">>> Auth middleware global");
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
                let auth_info = AuthInfo {
                    access_token: auth_token.token,
                    user_id: user.id,
                    email: user.email,
                    name: user.name,
                    status: user.status,
                    email_verified_at: user.email_verified_at.map(Into::into),
                };

                req.extensions_mut().insert(auth_info);
            }
        }
    }

    next.call(req).await
} // end function auth_middleware_global
//
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
