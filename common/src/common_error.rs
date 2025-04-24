#[derive(Debug, thiserror::Error)]
pub enum CommonError {
    #[error("ConfigError:: {0}")]
    ConfigError(String),
    #[error("PasswordError:: {0}")]
    PasswordError(String),
}
