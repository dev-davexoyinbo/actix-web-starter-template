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

impl AuthInfo {
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.iter().any(|p| p == permission)
    }
}
