use std::ops::Deref;

use actix_web::{FromRequest, HttpMessage, http::StatusCode};
use app_errors::UserError;
use futures_util::future::{LocalBoxFuture, Ready, ready};

use super::auth_models::AuthInfo;

pub struct AuthExtractor {
    value: AuthInfo,
}

impl AuthExtractor {
    pub fn into_inner(self) -> AuthInfo {
        self.value
    }
}

impl Deref for AuthExtractor {
    type Target = AuthInfo;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FromRequest for AuthExtractor {
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            let extensions = req.extensions();
            let auth_info = extensions.get::<AuthInfo>();

            let Some(auth_info) = auth_info else {
                let err = UserError::from_message("Unauthorized", StatusCode::UNAUTHORIZED);
                return Err(err.into());
            };

            Ok(AuthExtractor {
                value: auth_info.clone(),
            })
        })
    }
}

#[derive(Debug)]
pub struct MaybeAuthExtractor {
    value: Option<AuthInfo>,
}

impl MaybeAuthExtractor {
    pub fn _into_inner(self) -> Option<AuthInfo> {
        self.value
    }
}

impl Deref for MaybeAuthExtractor {
    type Target = Option<AuthInfo>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FromRequest for MaybeAuthExtractor {
    type Error = actix_web::Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let auth_info = req.extensions().get::<AuthInfo>().cloned();

        ready(Ok(MaybeAuthExtractor { value: auth_info }))
    }
}
