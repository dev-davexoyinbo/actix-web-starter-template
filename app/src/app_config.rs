use std::sync::OnceLock;

use actix_web::web;
use app_errors::AppError;
use common::AppConfig;

static APP_CONFIG_CELL: OnceLock<web::Data<AppConfig>> = OnceLock::new();
pub fn _get<'a>() -> Result<&'a AppConfig, AppError> {
    let x = APP_CONFIG_CELL.get();

    match x {
        Some(app_config) => Ok(app_config),
        None => Err(AppError::CustomError(
            "AppConfig not initialized".to_string(),
        )),
    }
} // end get_app_config

pub async fn initialize(app_config: web::Data<AppConfig>) -> Result<(), AppError> {
    APP_CONFIG_CELL
        .set(app_config)
        .map_err(|err| AppError::CustomError(format!("Error initializing AppConfig: {:?}", err)))?;

    Ok(())
} // end function initialize
