#[derive(Debug, thiserror::Error)]
pub enum MessagingError {
    #[error("{0}")]
    Custom(String),

    #[error("ClientCreation: {0}")]
    ClientCreationError(String),

    #[error("MessagingSendingError: {0}")]
    MessagingSendingError(String),

    #[error("MessagingDeliveryError: {0}")]
    MessagingDeliveryError(String),

    #[error("DeserializationError: {0}")]
    DeserializationError(#[from] serde_json::Error),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("CommitError: {0}")]
    CommitError(String),

    #[error("ShutdownError: {0}")]
    ShutdownError(String),

    #[error("TemplateError: {0}")]
    TemplateError(String),

    #[error("InitializationError: {0}")]
    InitializationError(String),

    #[error("AddressError: {0}")]
    AddressError(#[from] lettre::address::AddressError),
}
