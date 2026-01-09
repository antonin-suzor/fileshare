use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UploadStartRequest {
    pub file_name: String,
    pub expires_at: DateTime<FixedOffset>,
    pub content_type: String,
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub password: String,
}
