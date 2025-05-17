use serde::Serialize;

/// Response DTO for access control test endpoints
#[derive(Serialize)]
pub struct AccessControlTestResponseDto {
    pub message: String,
    pub endpoint_name: String,
    pub access_requirement: String,
}