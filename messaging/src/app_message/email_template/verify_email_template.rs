use askama::Template;
use serde::{Deserialize, Serialize};

use super::EmailTemplate;

#[derive(Template, Serialize, Deserialize, Debug, Clone)]
#[template(path = "verify-email.html")]
pub struct VerifyEmailTemplate {
    pub name: String,
    pub otp: String,
}

impl From<VerifyEmailTemplate> for EmailTemplate {
    fn from(val: VerifyEmailTemplate) -> EmailTemplate {
        EmailTemplate::VerifyEmail(val)
    }
}
