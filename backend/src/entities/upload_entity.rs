use chrono::{DateTime, FixedOffset};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Upload {
    pub id: Uuid,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub user_id: Option<Uuid>,
    pub file_name: String,
    pub content_type: String,
    pub presigned_get: String,
    pub expires_at: DateTime<FixedOffset>,
}
