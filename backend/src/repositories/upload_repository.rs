use crate::entities::Upload;
use chrono::{DateTime, FixedOffset, Utc};
use sqlx::{Error as SqlxError, PgPool};
use uuid::Uuid;

pub struct UploadRepository {}
impl UploadRepository {
    pub async fn list(db_pool: &PgPool) -> Result<Vec<Upload>, SqlxError> {
        let res: Vec<Upload> = sqlx::query_as("SELECT * FROM uploads;")
            .fetch_all(db_pool)
            .await?;
        Ok(res)
    }

    pub async fn from_id(db_pool: &PgPool, id: &Uuid) -> Result<Option<Upload>, SqlxError> {
        let res: Option<Upload> = sqlx::query_as("SELECT * FROM uploads WHERE id = $1 LIMIT 1;")
            .bind(id)
            .fetch_optional(db_pool)
            .await?;

        Ok(res)
    }
    pub async fn from_user_id(db_pool: &PgPool, user_id: &Uuid) -> Result<Vec<Upload>, SqlxError> {
        let res: Vec<Upload> = sqlx::query_as("SELECT * FROM uploads WHERE user_id = $1;")
            .bind(user_id)
            .fetch_all(db_pool)
            .await?;
        Ok(res)
    }

    pub async fn insert(
        db_pool: &PgPool,
        id: &Uuid,
        user_id: &Uuid,
        file_name: &str,
        content_type: &str,
        presigned_get: &str,
        expires_at: &DateTime<FixedOffset>,
    ) -> Result<Upload, SqlxError> {
        let res: Upload = sqlx::query_as("INSERT INTO uploads (id, user_id, file_name, content_type, presigned_get, expires_at) values ($1, $2, $3, $4, $5, $6) RETURNING *;")
            .bind(id)
            .bind(user_id)
            .bind(file_name)
            .bind(content_type)
            .bind(presigned_get)
            .bind(expires_at)
            .fetch_one(db_pool)
            .await?;
        Ok(res)
    }

    pub async fn delete_from_id(db_pool: &PgPool, id: &Uuid) -> Result<(), SqlxError> {
        let now = Utc::now().fixed_offset();
        sqlx::query("DELETE FROM uploads WHERE id = $2 RETURNING id;")
            .bind(now)
            .bind(id)
            .fetch_one(db_pool)
            .await?;
        Ok(())
    }
}
