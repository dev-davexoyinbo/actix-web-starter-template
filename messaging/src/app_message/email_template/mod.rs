mod hello_email_template;
use askama::Template;
pub use hello_email_template::*;
mod verify_email_template;
pub use verify_email_template::*;

use serde::{Deserialize, Serialize};

use crate::MessagingError;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum EmailTemplate {
    Hello(HelloEmailTemplate),
    VerifyEmail(VerifyEmailTemplate),
}

impl EmailTemplate {
    pub fn render(&self) -> Result<String, MessagingError> {
        match self {
            EmailTemplate::Hello(template) => template.render().map_err(|_| {
                MessagingError::TemplateError("Failed to render HelloEmailTemplate".to_string())
            }),
            EmailTemplate::VerifyEmail(template) => template.render().map_err(|_| {
                MessagingError::TemplateError("Failed to render VerifyEmailTemplate".to_string())
            }),
        }
    }
}
