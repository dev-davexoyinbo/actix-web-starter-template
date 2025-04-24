use actix_web::{HttpResponse, ResponseError, error::JsonPayloadError, http::StatusCode};
use serde_json::json;
use validator::ValidationErrors;

use crate::{AppError, Error};

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("{0}")]
    AppError(#[from] AppError),

    #[error("{0}")]
    CustomError(serde_json::Value, StatusCode),

    #[error("ValidationErrors: {0}")]
    ValidationErrors(#[from] ValidationErrors),

    #[error("JsonPayloadErrors: {0}")]
    JsonPayloadErrors(#[from] JsonPayloadError),
}

impl From<Error> for UserError {
    fn from(error: Error) -> Self {
        match error {
            Error::AppError(app_error) => UserError::AppError(app_error),
            Error::UserError(user_error) => user_error,
        }
    }
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        let val: serde_json::Value = match self {
            UserError::AppError(val) => json!({
                "message": val.to_string()
            }),
            UserError::JsonPayloadErrors(val) => json!({
                "message": val.to_string()
            }),
            UserError::ValidationErrors(err) => {
                json!({
                    "message": "An error occurred during validation",
                    "errors": err
                })
            }
            UserError::CustomError(value, _) => value.to_owned(),
        };

        HttpResponse::build(self.status_code()).json(val)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            UserError::AppError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::ValidationErrors(_) | UserError::JsonPayloadErrors(_) => {
                StatusCode::BAD_REQUEST
            }
            UserError::CustomError(_value, status) => status.to_owned(),
        }
    }
}

impl UserError {
    pub fn from_message(message: &str, status: StatusCode) -> Self {
        UserError::CustomError(json!({ "message": message }), status)
    }
}
