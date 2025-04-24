use common::MessagingConfig;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    MessagingError, app_message::AppMessageWrapper,
    messaging_consumer_wrapper::MessagingConsumerWrapper, messaging_producer::MessagingProducer,
};

pub struct MessagingClient {
    producer: MessagingProducer,
    consumer: Arc<RwLock<MessagingConsumerWrapper>>,
}

impl MessagingClient {
    pub async fn initialize(config: MessagingConfig) -> Result<Self, MessagingError> {
        Ok(MessagingClient {
            producer: MessagingProducer::initialize(config.clone())?,
            consumer: MessagingConsumerWrapper::initialize(config).await?,
        })
    } // end function initialize

    pub async fn send_message(&self, msg: AppMessageWrapper) -> Result<(), MessagingError> {
        self.producer.send(msg).await
    } // end function send_message

    pub async fn start_consumer(&mut self) -> Result<(), MessagingError> {
        self.consumer.write().await.start()
    } // end function start_consumer

    pub async fn stop_consumer(&mut self) -> Result<(), MessagingError> {
        self.consumer.write().await.stop().await
    } // end function start_consumer
} // end impl MessagingClient
