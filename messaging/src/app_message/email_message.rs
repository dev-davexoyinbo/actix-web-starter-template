use lettre::{SmtpTransport, Transport, message::header::ContentType};
use serde::{Deserialize, Serialize};

use crate::MessagingError;

use super::{AppMessage, email_template::EmailTemplate};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmailMessage {
    pub from: String,
    pub to: String,
    pub reply_to: Option<String>,
    pub subject: String,
    pub template: EmailTemplate,
}

impl From<EmailMessage> for AppMessage {
    fn from(val: EmailMessage) -> AppMessage {
        AppMessage::Email(val)
    }
}

impl EmailMessage {
    pub async fn process_email(&self, mailer: &SmtpTransport) -> Result<(), MessagingError> {
        let template_string = self.template.render()?;

        let email = lettre::Message::builder()
            .from(self.from.parse()?)
            // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
            .to(self.to.parse()?)
            .subject(&self.subject)
            .header(ContentType::TEXT_HTML)
            .body(template_string)
            .map_err(|err| {
                MessagingError::Custom(format!("Failed to create email message: {}", err))
            })?;

        match mailer.send(&email) {
            Ok(_) => {
                tracing::info!("Email sent successfully");
                Ok(())
            }
            Err(err) => {
                tracing::error!("Failed to send email: {}", err);
                Err(MessagingError::Custom(format!(
                    "Failed to send email: {}",
                    err
                )))
            }
        }
    }
}
