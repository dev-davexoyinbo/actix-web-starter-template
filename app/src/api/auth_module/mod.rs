mod auth_dtos;
pub mod auth_extractors;
pub mod auth_middleware;
pub mod auth_models;
mod auth_service;
mod handlers;

use actix_web::web::{self, ServiceConfig};

pub fn scoped_config(config: &mut ServiceConfig) {
    config.service(
        web::scope("/auth")
            .route("register", web::post().to(handlers::register))
            .route("login", web::post().to(handlers::login)),
    );
} // end function scoped_config
