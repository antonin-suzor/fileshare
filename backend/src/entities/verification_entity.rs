use chrono::{DateTime, FixedOffset};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Verification {
    pub id: Uuid,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub user_id: Option<Uuid>,
    pub activated_at: Option<DateTime<FixedOffset>>,
}
