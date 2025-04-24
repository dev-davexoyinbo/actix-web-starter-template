pub mod auth_module;
use actix_web::web::{self, ServiceConfig};

pub fn scoped_config(config: &mut ServiceConfig) {
    config.service(web::scope("/api").configure(auth_module::scoped_config));
} //end function scoped_config
