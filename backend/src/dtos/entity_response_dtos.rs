use crate::entities::{Upload, User};
use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub email: String,
    pub verified: bool,
}
impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            verified: value.is_verified(),
            email: value.email,
        }
    }
}
impl From<&User> for UserResponse {
    fn from(value: &User) -> Self {
        Self {
            id: value.id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            email: value.email.clone(),
            verified: value.is_verified(),
        }
    }
}

#[derive(Serialize)]
pub struct UploadResponse {
    pub id: Uuid,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub user_id: Option<Uuid>,
    pub file_name: String,
    pub content_type: String,
    pub presigned_get: String,
    pub expires_at: DateTime<FixedOffset>,
}
impl From<Upload> for UploadResponse {
    fn from(value: Upload) -> Self {
        Self {
            id: value.id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            user_id: value.user_id,
            file_name: value.file_name,
            content_type: value.content_type,
            presigned_get: value.presigned_get,
            expires_at: value.expires_at,
        }
    }
}
impl From<&Upload> for UploadResponse {
    fn from(value: &Upload) -> Self {
        Self {
            id: value.id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            user_id: value.user_id,
            file_name: value.file_name.clone(),
            content_type: value.content_type.clone(),
            presigned_get: value.presigned_get.clone(),
            expires_at: value.expires_at,
        }
    }
}
