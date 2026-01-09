use crate::entities::User;
use sqlx::{Error as SqlxError, PgPool};
use uuid::Uuid;

pub struct UserRepository {}
impl UserRepository {
    pub async fn list(db_pool: &PgPool) -> Result<Vec<User>, SqlxError> {
        let res: Vec<User> = sqlx::query_as("SELECT * FROM users;")
            .fetch_all(db_pool)
            .await?;
        Ok(res)
    }

    pub async fn from_id(db_pool: &PgPool, id: &Uuid) -> Result<Option<User>, SqlxError> {
        let res: Option<User> = sqlx::query_as("SELECT * FROM users WHERE id = $1 LIMIT 1;")
            .bind(id)
            .fetch_optional(db_pool)
            .await?;
        Ok(res)
    }

    pub async fn from_email(db_pool: &PgPool, email: &str) -> Result<Option<User>, SqlxError> {
        let res: Option<User> = sqlx::query_as("SELECT * FROM users WHERE email = $1 LIMIT 1;")
            .bind(email)
            .fetch_optional(db_pool)
            .await?;
        Ok(res)
    }

    pub async fn create(
        db_pool: &PgPool,
        email: &str,
        password_hash: &str,
    ) -> Result<User, SqlxError> {
        let res: User =
            sqlx::query_as("INSERT INTO users (email, password_hash) values ($1, $2) RETURNING *;")
                .bind(email)
                .bind(password_hash)
                .fetch_one(db_pool)
                .await?;
        Ok(res)
    }

    pub async fn set_user_verification(
        db_pool: &PgPool,
        user_id: &Uuid,
        verification_id: &Uuid,
    ) -> Result<Option<User>, SqlxError> {
        let res: Option<User> = sqlx::query_as(
            "UPDATE users SET updated_at = now(), verified_with_id = $1 WHERE id = $2 RETURNING *;",
        )
        .bind(verification_id)
        .bind(user_id)
        .fetch_optional(db_pool)
        .await?;
        Ok(res)
    }

    pub async fn update_password(
        db_pool: &PgPool,
        user_id: &Uuid,
        password_hash: &str,
    ) -> Result<Option<User>, SqlxError> {
        let res: Option<User> = sqlx::query_as(
            "UPDATE users SET updated_at = now(), password_hash = $1 WHERE id = $2 RETURNING *;",
        )
        .bind(password_hash)
        .bind(user_id)
        .fetch_optional(db_pool)
        .await?;
        Ok(res)
    }

    pub async fn delete_from_id(db_pool: &PgPool, id: &Uuid) -> Result<(), SqlxError> {
        sqlx::query("DELETE FROM users WHERE id = $1 RETURNING id;")
            .bind(id)
            .fetch_one(db_pool)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn from_email_contains_correct(pool: PgPool) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO users (email, password_hash) values ('correct', 'hash');")
            .execute(&pool)
            .await?;
        let from_email = UserRepository::from_email(&pool, "correct").await?;
        assert!(from_email.is_some());
        assert_eq!(from_email.unwrap().email, "correct");
        Ok(())
    }

    #[sqlx::test]
    async fn from_email_does_not_contain_incorrect(pool: PgPool) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO users (email, password_hash) values ('incorrect', 'hash');")
            .execute(&pool)
            .await?;
        let from_email = UserRepository::from_email(&pool, "correct").await?;
        assert!(from_email.is_none());
        Ok(())
    }
}
