use chrono::{DateTime, Utc};
use entity::sea_orm_active_enums::UserStatus;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RegisterRequestDto {
    #[validate(email(message = "Invalid email format"))]
    #[serde(default)]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    #[serde(default)]
    pub password: String,
    #[validate(length(min = 1, message = "Name must be at least 1 character long"))]
    #[serde(default)]
    pub name: String,
}

#[derive(Serialize, Debug)]
pub struct RegisterRequestResponseDto {
    pub id: i64,
}

#[derive(Deserialize, Debug, Validate)]
pub struct LoginRequestDto {
    #[validate(email(message = "Invalid email format"))]
    #[serde(default)]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    #[serde(default)]
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResponseDto {
    pub id: i64,
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct UserMeResponseDto {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub status: UserStatus,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct VerifyEmailRequestDto {
    #[validate(length(min = 1, message = "Token must be at least 1 character long"))]
    #[serde(default)]
    pub token: String,
    #[validate(email(message = "Invalid email format"))]
    #[serde(default)]
    pub email: String,
}

#[derive(Deserialize, Debug, Validate)]
pub struct ResendVerificationEmailRequestDto {
    #[validate(email(message = "Invalid email format"))]
    #[serde(default)]
    pub email: String,
}
