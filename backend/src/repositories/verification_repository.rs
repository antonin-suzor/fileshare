use crate::entities::Verification;
use chrono::Utc;
use sqlx::{Error as SqlxError, PgPool};
use uuid::Uuid;

pub struct VerificationRepository {}
impl VerificationRepository {
    pub async fn list(db_pool: &PgPool) -> Result<Vec<Verification>, SqlxError> {
        let res: Vec<Verification> = sqlx::query_as("SELECT * FROM verifications;")
            .fetch_all(db_pool)
            .await?;
        Ok(res)
    }

    pub async fn from_id(db_pool: &PgPool, id: &Uuid) -> Result<Option<Verification>, SqlxError> {
        let res: Option<Verification> =
            sqlx::query_as("SELECT * FROM verifications WHERE id = $1 LIMIT 1;")
                .bind(id)
                .fetch_optional(db_pool)
                .await?;

        Ok(res)
    }
    pub async fn from_user_id(
        db_pool: &PgPool,
        user_id: &Uuid,
    ) -> Result<Vec<Verification>, SqlxError> {
        let res: Vec<Verification> =
            sqlx::query_as("SELECT * FROM verifications WHERE user_id = $1;")
                .bind(user_id)
                .fetch_all(db_pool)
                .await?;
        Ok(res)
    }

    pub async fn insert(db_pool: &PgPool, user_id: &Uuid) -> Result<Verification, SqlxError> {
        let res: Verification =
            sqlx::query_as("INSERT INTO verifications (user_id) values ($1) RETURNING *;")
                .bind(user_id)
                .fetch_one(db_pool)
                .await?;
        Ok(res)
    }

    pub async fn set_activated_at_now(db_pool: &PgPool, id: &Uuid) -> Result<(), SqlxError> {
        let now = Utc::now().fixed_offset();
        sqlx::query("UPDATE verifications SET updated_at = $1, activated_at = $2 WHERE id = $3 RETURNING id;")
            .bind(now)
            .bind(now)
            .bind(id)
            .fetch_one(db_pool)
            .await?;
        Ok(())
    }

    pub async fn delete_from_id(db_pool: &PgPool, id: &Uuid) -> Result<(), SqlxError> {
        let now = Utc::now().fixed_offset();
        sqlx::query("UPDATE verifications SET deleted_at = $1 WHERE id = $2 RETURNING id;")
            .bind(now)
            .bind(id)
            .fetch_one(db_pool)
            .await?;
        Ok(())
    }
}
