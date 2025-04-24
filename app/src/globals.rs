use std::sync::OnceLock;

use actix_web::web;
use app_errors::AppError;

pub mod password {
    use super::*;
    use common::PasswordUtils;

    static CELL: OnceLock<web::Data<PasswordUtils>> = OnceLock::new();
    pub fn get() -> Result<web::Data<PasswordUtils>, AppError> {
        match CELL.get() {
            Some(rv) => Ok(rv.clone()),
            None => Err(AppError::CustomError(
                "Global password utils has not been initialized".to_string(),
            )),
        }
    } // end method get

    pub fn set(val: web::Data<PasswordUtils>) -> Result<(), AppError> {
        CELL.set(val).map_err(|_| {
            AppError::CustomError("Error setting value of global password utils".to_string())
        })?;

        Ok(())
    } // end method set
}

pub mod messaging {
    use ::messaging::MessagingClient;
    use tokio::sync::RwLock;

    use super::*;

    static CELL: OnceLock<web::Data<RwLock<MessagingClient>>> = OnceLock::new();
    pub fn get() -> Result<web::Data<RwLock<MessagingClient>>, AppError> {
        match CELL.get() {
            Some(rv) => Ok(rv.clone()),
            None => Err(AppError::CustomError(
                "Messaging client has not been initialized".to_string(),
            )),
        }
    } // end method get

    pub fn set(val: web::Data<RwLock<MessagingClient>>) -> Result<(), AppError> {
        CELL.set(val).map_err(|_| {
            AppError::CustomError("Error setting value of messaging client".to_string())
        })?;

        Ok(())
    } // end method set
}
