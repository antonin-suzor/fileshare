use crate::dtos::UserResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct CountResponse {
    pub count: i32,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct SignUpResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Serialize)]
pub struct VerifyResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Serialize)]
pub struct UploadStartResponse {
    pub url: String,
}
