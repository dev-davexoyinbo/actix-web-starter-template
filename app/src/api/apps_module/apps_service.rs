use actix_web::http::StatusCode;
use celeris_errors::{AppError, Error, UserError};
use entity::accounts;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::persistence_state;

use super::apps_dto::{CreateAppRequestDto, CreateAppResponseDto};

pub struct AppsService;

impl AppsService {
    pub async fn create_app(
        account_id: i64,
        dto: CreateAppRequestDto,
    ) -> Result<CreateAppResponseDto, Error> {
        let persistence_state = persistence_state::get()?;
        let db = &persistence_state.db;

        let account = accounts::Entity::find()
            .filter(accounts::Column::Id.eq(account_id))
            .one(db)
            .await
            .map_err(AppError::DbErr)?;

        let Some(account) = account else {
            let err = UserError::from_message("Account not found", StatusCode::NOT_FOUND);
            return Err(err.into());
        };

        todo!("Implement the create_app method in AppsService")
    } // end create_app
} // end impl AppsService
