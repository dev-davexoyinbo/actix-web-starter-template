use actix_web::{
    HttpResponse, Responder,
    web::{self, Json},
};
use app_errors::UserError;
use common::ResponseDto;
use tracing::instrument;
use validator::Validate;

use super::{
    auth_dtos::{LoginRequestDto, RegisterRequestDto, RegisterRequestResponseDto},
    auth_service::AuthService,
};

#[instrument(skip_all)]
pub async fn register(
    dto: Json<RegisterRequestDto>,
) -> Result<web::Json<ResponseDto<RegisterRequestResponseDto>>, UserError> {
    dto.validate()?;

    let dto = AuthService::register(dto.into_inner()).await?;
    let res = web::Json(ResponseDto::new(
        "User registered successfully".to_string(),
        dto,
    ));

    Ok(res)
} // end function register

#[instrument]
pub async fn login(dto: Json<LoginRequestDto>) -> Result<impl Responder, UserError> {
    dto.validate()?;

    let dto = AuthService::login(dto.into_inner()).await?;
    let res = ResponseDto::new("User logged in successfully".to_string(), dto);

    Ok(HttpResponse::Created().json(res))
} // end function login
