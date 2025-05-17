use actix_web::{
    HttpResponse, Responder,
    web::{self, Json},
};
use app_errors::UserError;
use common::ResponseDto;
use tracing::instrument;
use validator::Validate;

use super::{
    auth_dtos::{
        LoginRequestDto, RegisterRequestDto, RegisterRequestResponseDto,
        ResendVerificationEmailRequestDto, UserMeResponseDto, VerifyEmailRequestDto,
    },
    auth_extractors::AuthExtractor,
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

#[instrument(skip_all)]
pub async fn login(dto: Json<LoginRequestDto>) -> Result<impl Responder, UserError> {
    dto.validate()?;

    let dto = AuthService::login(dto.into_inner()).await?;
    let res = ResponseDto::new("User logged in successfully".to_string(), dto);

    Ok(HttpResponse::Created().json(res))
} // end function login

#[instrument(skip_all)]
pub async fn me(
    auth_info: AuthExtractor,
) -> Result<web::Json<ResponseDto<UserMeResponseDto>>, UserError> {
    let auth_info = auth_info.into_inner();

    let dto = AuthService::me(auth_info.user_id).await?;

    Ok(web::Json(ResponseDto::new("".to_string(), dto)))
}

#[instrument(skip_all)]
pub async fn verify_email(
    dto: web::Json<VerifyEmailRequestDto>,
) -> Result<impl Responder, UserError> {
    dto.validate()?;

    AuthService::verify_email(dto.into_inner()).await?;

    let res = ResponseDto::new("Email verified successfully".to_string(), ());

    Ok(web::Json(res))
} // end function verify_email

#[instrument(skip_all)]
pub async fn resend_verification_email(
    dto: web::Json<ResendVerificationEmailRequestDto>,
) -> Result<impl Responder, UserError> {
    dto.validate()?;

    AuthService::resend_verification_email(dto.into_inner()).await?;

    let res = ResponseDto::new("Verification email resent successfully".to_string(), ());

    Ok(web::Json(res))
} // end function resend_verification_email
