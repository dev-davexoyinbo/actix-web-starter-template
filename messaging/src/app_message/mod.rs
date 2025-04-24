mod app_message_topic;
use std::ops::Deref;

pub use app_message_topic::*;
use serde::{Deserialize, Serialize};

mod email_message;
pub use email_message::*;

mod email_template;
pub use email_template::*;

pub struct AppMessageWrapper {
    pub key: Option<String>,
    pub topic: AppMessageTopic,
    pub message: AppMessage,
}

impl Deref for AppMessageWrapper {
    type Target = AppMessage;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppMessage {
    Email(EmailMessage),
}
