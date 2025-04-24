use entity::sea_orm_active_enums::AppStatus;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateAppRequestDto {
    #[validate(length(min = 1, message = "Name must be at least 1 character long"))]
    #[serde(default)]
    pub name: String,
    #[validate(length(min = 1, message = "Description must be at least 1 character long"))]
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub status: Option<AppStatus>,
}

#[derive(Debug, Clone, Serialize, Validate)]
pub struct CreateAppResponseDto {
    pub id: i64,
}
