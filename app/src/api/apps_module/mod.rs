use actix_web::web::{self, ServiceConfig};

mod apps_dto;
mod apps_handlers;
mod apps_service;

pub fn scoped_config(config: &mut ServiceConfig) {
    config.service(web::scope("/apps").route("", web::post().to(apps_handlers::create_app)));
} // end function scoped_config
