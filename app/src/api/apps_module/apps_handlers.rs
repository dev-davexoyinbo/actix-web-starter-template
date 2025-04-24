use actix_web::{Responder, web};
use celeris_errors::UserError;

use crate::api::auth_module::auth_extractors::AuthExtractor;

use super::apps_dto::CreateAppRequestDto;

pub async fn create_app(
    auth_info: AuthExtractor,
    dto: web::Json<CreateAppRequestDto>,
) -> Result<impl Responder, UserError> {
    let auth_info = auth_info.into_inner();

    Ok("This is the create app handler")
} // end method create_app
