mod access_control_test_dtos;
mod access_control_test_handlers;
mod auth_dtos;
pub mod auth_extractors;
pub mod auth_models;
mod auth_service;
mod handlers;

use actix_web::web::{self, ServiceConfig};

pub fn scoped_config(config: &mut ServiceConfig) {
    // Standard authentication routes
    config.service(
        web::scope("/auth")
            .route("register", web::post().to(handlers::register))
            .route("login", web::post().to(handlers::login))
            .route("me", web::get().to(handlers::me))
            .route("verify-email", web::post().to(handlers::verify_email))
            .route(
                "resend-verification-email",
                web::post().to(handlers::resend_verification_email),
            ),
    );

    // Example files for access control.
    access_control_test_handlers::configure_access_control_routes(config);
} // end function scoped_config
