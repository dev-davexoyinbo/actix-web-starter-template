use common::MessagingConfig;
use std::sync::Arc;
use tokio::sync::RwLock;

use lettre::{SmtpTransport, transport::smtp::authentication::Credentials};

use crate::{AppMessage, AppMessageTopic, MessagingError, messaging_consumer::MessagingConsumer};

pub struct MessagingConsumerWrapper {
    list: Vec<MessagingConsumer>,
    mailer: lettre::SmtpTransport,
}

impl MessagingConsumerWrapper {
    pub async fn initialize(config: MessagingConfig) -> Result<Arc<RwLock<Self>>, MessagingError> {
        let MessagingConfig {
            smtp_port,
            smtp_host,
            smtp_password,
            smtp_username,
            ..
        } = config.clone();

        let cred = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());
        let url = format!("smtp://{smtp_username}:{smtp_password}@{smtp_host}:{smtp_port}");

        let mailer = SmtpTransport::from_url(&url)
            .map_err(|err| {
                MessagingError::InitializationError(format!("Failed to create mailer: {}", err))
            })?
            .credentials(cred)
            .build();
        let topic_groups: Vec<Vec<AppMessageTopic>> = vec![
            vec![AppMessageTopic::GeneralEmail],
            vec![AppMessageTopic::PriorityEmail],
        ];

        let wrapper = MessagingConsumerWrapper {
            list: Vec::with_capacity(topic_groups.len()),
            mailer,
        };

        let wrapper = Arc::new(RwLock::new(wrapper));

        for topics in topic_groups.into_iter() {
            wrapper.write().await.list.push(
                MessagingConsumer::initialize(Arc::downgrade(&wrapper), topics, config.clone())
                    .await?,
            );
        }

        Ok(wrapper)
    }

    pub fn start(&mut self) -> Result<(), MessagingError> {
        for consumer in self.list.iter_mut() {
            consumer.start()?;
        }

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), MessagingError> {
        for consumer in self.list.iter_mut() {
            consumer.stop().await?;
        }

        Ok(())
    }

    pub async fn process_message(&self, message: AppMessage) -> Result<(), MessagingError> {
        match message {
            AppMessage::Email(email_message) => {
                email_message.process_email(&self.mailer).await?;
            }
        }
        Ok(())
    }
}
