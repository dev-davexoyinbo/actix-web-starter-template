mod hello_email_template;
use askama::Template;
pub use hello_email_template::*;

use serde::{Deserialize, Serialize};

use crate::MessagingError;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum EmailTemplate {
    Custom,
    Hello(HelloEmailTemplate),
}

impl EmailTemplate {
    pub fn render(&self) -> Result<String, MessagingError> {
        match self {
            EmailTemplate::Custom => Ok("Custom".to_string()),
            EmailTemplate::Hello(template) => template.render().map_err(|_| {
                MessagingError::TemplateError("Failed to render HelloEmailTemplate".to_string())
            }),
        }
    }
}
