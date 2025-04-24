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
