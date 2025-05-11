mod api;
mod app_config;
mod globals;
mod handlers;
mod persistence_state;
mod telemetry;

use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    App, HttpServer,
    dev::Server,
    middleware::{NormalizePath, TrailingSlash, from_fn},
    web,
};
use api::auth_module::auth_middleware::auth_middleware_global;
use app_errors::{AppError, UserError};
use common::AppConfig;
use handlers::health_check;
use messaging::MessagingClient;
use persistence_state::PersistenceState;
use tokio::sync::RwLock;
use tracing_actix_web::TracingLogger;

pub async fn run_app() -> Result<Server, AppError> {
    // initialize telemetry
    telemetry::initialize()?;

    // initialize configurations
    tracing::info!(">>> Initializing app configurations...");
    let config = AppConfig::initialize().await?;
    let config = web::Data::new(config);
    app_config::initialize(config.clone()).await?;
    tracing::info!("<<< App configurations initialized successfully.");

    let host = config.app.host.clone();
    let port = config.app.port;

    // initialize persistence state
    tracing::info!(">>> Initializing persistence state(db)...");
    let p_state = PersistenceState::initialize(&config.db.url).await?;
    let p_state = web::Data::new(p_state);
    persistence_state::set(p_state.clone())?;
    tracing::info!("<<< Persistence state (db) initialized successfully.");

    tracing::info!(">>> Initializing password utils...");
    let password_utils = common::PasswordUtils::initialize(config.password.secret.clone())?;
    let password_utils = web::Data::new(password_utils);
    globals::password::set(password_utils.clone())?;
    tracing::info!("<<< Password utils initialized successfully.");

    tracing::info!(">>> Initializing messaging client...");
    let mut messaging_client = MessagingClient::initialize(config.messaging.clone()).await?;
    messaging_client.start_consumer().await?;
    let messaging_client = web::Data::new(RwLock::new(messaging_client));
    globals::messaging::set(messaging_client.clone())?;
    tracing::info!("<<< Messaing client initialized successfully.");

    let json_config = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| {
            tracing::error!("Error deserializing JSON: {}", err);
            let err: UserError = err.into();

            err.into()
        });

    let governor_conf = GovernorConfigBuilder::default()
        .requests_per_second(2)
        .burst_size(10)
        .finish()
        .ok_or_else(|| AppError::CustomError("Failed to create GovernorConfig".to_string()))?;

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);

        App::new()
            .wrap(from_fn(auth_middleware_global))
            .wrap(cors)
            .wrap(TracingLogger::default())
            .wrap(Governor::new(&governor_conf))
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .app_data(json_config.clone())
            .app_data(config.clone())
            .app_data(p_state.clone())
            .app_data(password_utils.clone())
            .app_data(messaging_client.clone())
            .route("/health-check", web::get().to(health_check))
            .configure(api::scoped_config)
    })
    .bind((host, port))?
    .run();

    Ok(server)
} // end function run_app
