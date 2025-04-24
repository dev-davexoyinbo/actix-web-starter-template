use celeris_errors::AppError;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    let server = celeris::run_app().await?;
    server.await.map_err(AppError::IOError)
}
