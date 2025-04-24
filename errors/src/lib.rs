mod app_error;
pub use app_error::*;

mod user_error;
pub use user_error::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    AppError(#[from] AppError),
    #[error("{0}")]
    UserError(#[from] UserError),
}

impl From<sea_orm::TransactionError<Error>> for Error {
    fn from(value: sea_orm::TransactionError<Error>) -> Self {
        match value {
            sea_orm::TransactionError::Connection(err) => Self::AppError(AppError::DbErr(err)),
            sea_orm::TransactionError::Transaction(err) => err,
        }
    }
}
