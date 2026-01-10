use sqlx::FromRow;
use uuid::Uuid;

pub mod upload_repository;
pub mod user_repository;
pub mod verification_repository;

pub use upload_repository::UploadRepository;
pub use user_repository::UserRepository;
pub use verification_repository::VerificationRepository;

#[derive(Debug, FromRow)]
pub struct ReturningId {
    pub id: Uuid,
}
impl From<Uuid> for ReturningId {
    fn from(value: Uuid) -> Self {
        Self { id: value }
    }
}

#[derive(Debug, FromRow)]
pub struct ReturningCount {
    pub count: i64,
}
impl From<i64> for ReturningCount {
    fn from(value: i64) -> Self {
        Self { count: value }
    }
}
