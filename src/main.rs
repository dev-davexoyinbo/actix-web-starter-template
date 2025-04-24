use app_errors::AppError;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    let server = app::run_app().await?;
    server.await.map_err(AppError::IOError)
}
