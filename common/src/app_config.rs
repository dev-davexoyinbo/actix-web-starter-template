use std::{env, str::FromStr};

use crate::common_error::CommonError;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub host: String,
    pub db: DbConfig,
    pub password: PasswordConfig,
    pub messaging: MessagingConfig,
}

#[derive(Debug, Clone)]
pub struct PasswordConfig {
    pub secret: String,
}

#[derive(Debug, Clone)]
pub struct MessagingConfig {
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_host: String,
    pub smtp_port: String,
    pub kafka_server_url: String,
}

impl PasswordConfig {
    pub fn initialize() -> Result<Self, CommonError> {
        let secret: String = get_required_env_val("APP_SECRET")?;

        if secret.len() < 5 {
            return Err(CommonError::ConfigError(
                "App secret too short, must have at least 5 characters".to_string(),
            ));
        } else if secret.len() > 48 {
            return Err(CommonError::ConfigError(
                "App secret too long, must not be greater than 48 characters".to_string(),
            ));
        }

        Ok(Self { secret })
    }
}

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub url: String,
}

impl AppConfig {
    pub async fn initialize() -> Result<Self, CommonError> {
        dotenv::dotenv().ok();

        let rv = Self {
            port: get_optional_env_val("APP_PORT", 8080),
            host: get_optional_env_val("APP_HOST", "0.0.0.0".to_string()),
            db: DbConfig {
                url: get_required_env_val("DATABASE_URL")?,
            },
            password: PasswordConfig::initialize()?,
            messaging: MessagingConfig {
                smtp_username: get_required_env_val("SMTP_USERNAME")?,
                smtp_password: get_required_env_val("SMTP_PASSWORD")?,
                smtp_host: get_required_env_val("SMTP_HOST")?,
                smtp_port: get_required_env_val("SMTP_PORT")?,
                kafka_server_url: get_required_env_val("APP_KAFKA_SERVER")?,
            },
        };

        Ok(rv)
    }
}

fn get_optional_env_val<R>(key: &str, default_value: R) -> R
where
    R: FromStr,
{
    match env::var(key) {
        Ok(val) => val.parse::<R>().unwrap_or(default_value),
        Err(_) => default_value,
    }
}

fn get_required_env_val<R>(key: &str) -> Result<R, CommonError>
where
    R: FromStr,
{
    let val = env::var(key).map_err(|err| {
        CommonError::ConfigError(format!(
            "Error retrieving env variable {key} from env. {}",
            err
        ))
    })?;
    val.parse::<R>()
        .map_err(|_err| CommonError::ConfigError(format!("Error parsing {key} from env. {val}")))
}
