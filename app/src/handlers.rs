use actix_web::{Responder, web};
use app_errors::UserError;
use serde_json::json;
use tracing::instrument;

#[instrument(skip_all)]
pub async fn health_check() -> Result<impl Responder, UserError> {
    // let app_message_wrapper = AppMessageWrapper {
    //     key: None,
    //     topic: AppMessageTopic::PriorityEmail,
    //     message: messaging::EmailMessage {
    //         from: "david@email.com".to_string(),
    //         to: "priority@email.com".to_string(),
    //         reply_to: None,
    //         subject: "Subject".to_string(),
    //         template: HelloEmailTemplate {
    //             name: "David".to_string(),
    //         }
    //         .into(),
    //     }
    //     .into(),
    // };
    //
    // messaging_client
    //     .read()
    //     .await
    //     .send_message(app_message_wrapper)
    //     .await
    //     .map_err(AppError::MessagingError)?;
    //
    // let app_message_wrapper = AppMessageWrapper {
    //     key: None,
    //     topic: AppMessageTopic::GeneralEmail,
    //     message: messaging::EmailMessage {
    //         from: "david@email.com".to_string(),
    //         to: "general@email.com".to_string(),
    //         reply_to: None,
    //         subject: "Subject".to_string(),
    //         template: HelloEmailTemplate {
    //             name: "David".to_string(),
    //         }
    //         .into(),
    //     }
    //     .into(),
    // };
    //
    // messaging_client
    //     .read()
    //     .await
    //     .send_message(app_message_wrapper)
    //     .await
    //     .map_err(AppError::MessagingError)?;
    //
    tracing::info!("Health check");
    Ok(web::Json(json!({"message": "Alive"})))
}
