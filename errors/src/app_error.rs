use argon2::password_hash;
use common::common_error::CommonError;
use messaging::MessagingError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),
    #[error("VarError: {0}")]
    VarError(#[from] std::env::VarError),
    #[error("{0}")]
    CustomError(String),
    #[error("DbErr: {0}")]
    DbErr(#[from] sea_orm::DbErr),
    #[error("MessagingError: {0}")]
    MessagingError(#[from] MessagingError),
    #[error("CommonError:{0}")]
    CommonError(#[from] CommonError),
}

impl From<password_hash::Error> for AppError {
    fn from(value: password_hash::Error) -> Self {
        Self::CustomError(format!("PasswordHash: {}", value))
    }
}
