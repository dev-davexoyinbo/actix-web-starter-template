use std::{sync::OnceLock, time::Duration};

use actix_web::web;
use app_errors::AppError;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub struct PersistenceState {
    pub db: DatabaseConnection,
}

impl PersistenceState {
    pub async fn initialize(db_url: &str) -> Result<Self, AppError> {
        let mut opt = ConnectOptions::new(db_url);

        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false)
            .sqlx_logging_level(log::LevelFilter::Info)
            .set_schema_search_path("public");

        let db = Database::connect(opt).await?;

        Ok(Self { db })
    } // end method initialize
} // end PersistenceState

static CELL: OnceLock<web::Data<PersistenceState>> = OnceLock::new();
pub fn get() -> Result<web::Data<PersistenceState>, AppError> {
    match CELL.get() {
        Some(rv) => Ok(rv.clone()),
        None => Err(AppError::CustomError(
            "Persistence state has not been initialized".to_string(),
        )),
    }
} // end method get

pub fn set(val: web::Data<PersistenceState>) -> Result<(), AppError> {
    CELL.set(val).map_err(|_| {
        AppError::CustomError("Error setting value of global persistence state".to_string())
    })?;

    Ok(())
} // end method set
