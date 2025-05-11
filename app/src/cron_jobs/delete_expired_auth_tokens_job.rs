use actix_jobs::Job;
use app_errors::AppError;
use entity::auth_tokens;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use tracing::instrument;

use crate::persistence_state;

pub struct DeleteExpiredAuthTokensJob;

impl Job for DeleteExpiredAuthTokensJob {
    fn cron(&self) -> &str {
        "0 * * * * *"
    }

    #[instrument(skip(self))]
    fn run(&mut self) {
        actix_web::rt::spawn(async {
            let db = persistence_state::get();

            let db = match &db {
                Ok(val) => &val.db,
                Err(err) => {
                    tracing::error!("Failed to get persistence state: {}", err);
                    return;
                }
            };

            let result = auth_tokens::Entity::delete_many()
                .filter(
                    Condition::all()
                        .add(auth_tokens::Column::ExpiresAt.is_not_null())
                        .add(auth_tokens::Column::ExpiresAt.lt(chrono::Utc::now())),
                )
                .exec(db)
                .await
                .map_err(AppError::DbErr);

            match result {
                Ok(_) => {
                    tracing::info!("Expired auth tokens deleted successfully.");
                }
                Err(err) => {
                    tracing::error!("Failed to delete expired auth tokens: {}", err);
                }
            };
        });
    } // end run
}
