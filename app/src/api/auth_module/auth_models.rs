use chrono::{DateTime, Utc};
use entity::sea_orm_active_enums::AccountStatus;

#[derive(Debug, PartialEq, Clone)]
pub struct AuthInfo {
    pub access_token: String,
    pub account_id: i64,
    pub email: String,
    pub name: String,
    pub status: AccountStatus,
    pub email_verified_at: Option<DateTime<Utc>>,
}
