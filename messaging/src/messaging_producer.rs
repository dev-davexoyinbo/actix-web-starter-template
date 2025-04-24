use std::time::Duration;

use common::MessagingConfig;
use rdkafka::{
    ClientConfig,
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
};

use crate::{MessagingError, app_message::AppMessageWrapper};

pub struct MessagingProducer {
    producer: FutureProducer,
}

impl MessagingProducer {
    pub fn initialize(config: MessagingConfig) -> Result<Self, MessagingError> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", config.kafka_server_url)
            .set("compression.type", "gzip")
            .set("retries", "3")
            .set("delivery.timeout.ms", "4000")
            .set("request.timeout.ms", "4000")
            .set("request.required.acks", "all")
            .set("queue.buffering.max.messages", "100000")
            .create()
            .map_err(|err| {
                MessagingError::ClientCreationError(format!("Error creating producer: {}", err))
            })?;

        Ok(Self { producer })
    }

    pub async fn send(&self, msg: AppMessageWrapper) -> Result<(), MessagingError> {
        let val = serde_json::to_string(&msg.message)?;
        let topic = msg.topic.to_string();

        let mut record = FutureRecord::to(&topic).payload(&val);

        if let Some(key) = &msg.key {
            record = record.key(key);
        }

        self.producer
            .send(record, Timeout::After(Duration::from_millis(2000)))
            .await
            .map_err(|(err, _)| MessagingError::MessagingSendingError(err.to_string()))?;

        Ok(())
    }
}
