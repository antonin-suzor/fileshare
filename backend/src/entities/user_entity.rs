use chrono::{DateTime, FixedOffset};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub email: String,
    pub password_hash: String,
    pub verified_with_id: Option<Uuid>,
}
impl User {
    pub fn is_verified(&self) -> bool {
        self.verified_with_id.is_some()
    }
}
