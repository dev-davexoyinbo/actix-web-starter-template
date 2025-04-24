use celeris_errors::AppError;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn initialize() -> Result<(), AppError> {
    initialize_tracing()?;
    Ok(())
}

fn initialize_tracing() -> Result<(), AppError> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).map_err(|e| {
        AppError::CustomError(format!("Setting default subscriber failed: {:?}", e))
    })?;

    Ok(())
}
