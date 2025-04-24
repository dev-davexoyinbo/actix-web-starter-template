use std::sync::{Arc, Weak};

use common::MessagingConfig;
use rdkafka::{
    ClientConfig, Message,
    consumer::{self, Consumer, StreamConsumer},
    message::BorrowedMessage,
};
use tokio::{
    sync::{RwLock, watch},
    task::JoinHandle,
};

use crate::{
    AppMessage, MessagingError, app_message::AppMessageTopic,
    messaging_consumer_wrapper::MessagingConsumerWrapper,
};

pub struct MessagingConsumer {
    topics: Vec<AppMessageTopic>,
    consumer: Arc<StreamConsumer>,
    shutdown_signal: watch::Receiver<()>,
    shutdown_sender: watch::Sender<()>,
    running_join_handle: Option<JoinHandle<Result<(), MessagingError>>>,
    wrapper_ref: Weak<RwLock<MessagingConsumerWrapper>>,
}

impl MessagingConsumer {
    pub async fn initialize(
        wrapper_ref: Weak<RwLock<MessagingConsumerWrapper>>,
        topics: Vec<AppMessageTopic>,
        config: MessagingConfig,
    ) -> Result<MessagingConsumer, MessagingError> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", config.kafka_server_url)
            .set("group.id", "celeris_group_id")
            .set("enable.auto.commit", "false")
            .set("auto.offset.reset", "earliest")
            .set("session.timeout.ms", "6000")
            .set("max.poll.interval.ms", "300000")
            .create()
            .map_err(|err| {
                MessagingError::ClientCreationError(format!("Error creating consumer: {}", err))
            })?;

        let (shutdown_sender, shutdown_signal) = watch::channel(());

        Ok(MessagingConsumer {
            topics,
            consumer: Arc::new(consumer),
            shutdown_signal,
            shutdown_sender,
            running_join_handle: None,
            wrapper_ref,
        })
    }

    pub fn start(&mut self) -> Result<(), MessagingError> {
        if self.running_join_handle.is_some() {
            return Ok(());
        }

        let topics: Vec<String> = self.topics.iter().map(|val| val.to_string()).collect();
        let topics: Vec<&str> = topics.iter().map(|val| val.as_str()).collect();

        self.consumer.subscribe(&topics).map_err(|err| {
            MessagingError::ClientCreationError(format!("Error subscribing to topic: {}", err))
        })?;

        let consumer = self.consumer.clone();
        let wrapper_ref = self.wrapper_ref.clone();
        let mut shutdown_signal = self.shutdown_signal.clone();

        let consumer_handle = tokio::spawn(async move {
            async fn handle_received_message(
                wrapper_ref: Weak<RwLock<MessagingConsumerWrapper>>,
                borrowed_message: BorrowedMessage<'_>,
                consumer: Arc<StreamConsumer>,
            ) -> Result<(), MessagingError> {
                let payload = match borrowed_message.payload_view::<str>() {
                    None => {
                        tracing::info!("None message");
                        return Ok(());
                    }
                    Some(Err(err)) => {
                        tracing::error!("Error parsing a message: {}", err);
                        return Ok(());
                    }
                    Some(Ok(payload)) => payload,
                };

                let app_message = match serde_json::from_str::<AppMessage>(payload) {
                    Err(err) => {
                        tracing::error!(err = ?err, "Error deserializing message");
                        return Ok(());
                    }
                    Ok(msg) => msg,
                };

                let Some(wrapper_ref) = wrapper_ref.upgrade() else {
                    tracing::error!("Failed to upgrade weak reference to wrapper");
                    return Ok(());
                };

                tracing::info!("Message received");

                let processed_message = wrapper_ref.read().await.process_message(app_message).await;
                match processed_message {
                    Ok(_) => {
                        tracing::info!("Message processed successfully");
                        consumer
                            .commit_message(&borrowed_message, consumer::CommitMode::Async)
                            .map_err(|err| MessagingError::CommitError(err.to_string()))?;
                    }
                    Err(err) => {
                        tracing::error!(err = ?err, "Error processing message");
                    }
                }

                Ok(())
            } // end function handle_received_message

            loop {
                tokio::select! {
                    _ = shutdown_signal.changed() => {
                        tracing::info!("Shutdown signal received, stopping consumer.");
                        break;
                    }

                    message = consumer.recv() => {
                        match message {
                            Err(err) => {
                                tracing::error!("Error receiving message: {:?}", err);
                            }
                            Ok(borrowed_message) => {
                                if let Err(err) = handle_received_message(wrapper_ref.clone(), borrowed_message, consumer.clone()).await {
                                    tracing::error!("Error handling message: {:?}", err);
                                }

                            }
                        }
                    }
                }
            }

            Result::<(), MessagingError>::Ok(())
        });

        self.running_join_handle = Some(consumer_handle);

        Ok(())
    } // end function start_consumer

    pub async fn stop(&mut self) -> Result<(), MessagingError> {
        if let Some(handle) = self.running_join_handle.take() {
            self.shutdown_sender.send(()).map_err(|_| {
                MessagingError::ShutdownError("Failed to send shutdown signal".to_string())
            })?;

            handle.await.map_err(|_| {
                MessagingError::ShutdownError("Failed to await consumer shutdown".to_string())
            })??;
        }

        Ok(())
    } // end function stop_consumer
}
