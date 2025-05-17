use chrono::{DateTime, Utc};
use entity::sea_orm_active_enums::UserStatus;

#[derive(Debug, PartialEq, Clone)]
pub struct AuthInfo {
    pub access_token: String,
    pub user_id: i64,
    pub email: String,
    pub name: String,
    pub status: UserStatus,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub permissions: Vec<String>,
    pub roles: Vec<String>,
}
