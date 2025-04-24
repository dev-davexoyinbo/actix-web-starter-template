use serde_json::Value;

mod common;

#[actix_web::test]
async fn test_health_check() {
    let val = common::spawn_app().await;
    let base_url = &val.base_url;

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(base_url.join("/health-check").expect("Failed to parse URL"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert status code
    assert!(response.status().is_success());
    assert_eq!(response.status().as_u16(), 200);

    // Assert content type
    let content_type = response
        .headers()
        .get("content-type")
        .expect("Response missing content-type header")
        .to_str()
        .expect("Failed to parse content-type header");
    assert_eq!(content_type, "application/json");

    // Assert response body
    let body: Value = response
        .json()
        .await
        .expect("Failed to parse response body as JSON");
    assert!(
        body.get("message").is_some(),
        "Response body is missing the 'message' key"
    );
} // end test_health_check
