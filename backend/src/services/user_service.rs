use crate::{
    entities::User,
    repositories::{UserRepository, VerificationRepository},
    services::{DiscordService, EmailService},
};
use anyhow::Context;
use sqlx::{Error as SqlxError, PgPool};
use uuid::Uuid;

pub struct UserService {}
impl UserService {
    pub async fn list(db_pool: &PgPool) -> Result<Vec<User>, SqlxError> {
        UserRepository::list(db_pool).await
    }

    pub async fn from_id(db_pool: &PgPool, id: &Uuid) -> Result<Option<User>, SqlxError> {
        UserRepository::from_id(db_pool, id).await
    }

    pub async fn from_email(db_pool: &PgPool, email: &str) -> Result<Option<User>, SqlxError> {
        UserRepository::from_email(db_pool, email).await
    }

    pub async fn signup(db_pool: &PgPool, email: &str, password: &str) -> anyhow::Result<User> {
        let user = UserRepository::create(
            db_pool,
            email,
            &bcrypt::hash(password, bcrypt::DEFAULT_COST)
                .with_context(|| "Failed to hash password")?,
        )
        .await?;

        Self::start_email_verification_process(db_pool, &user).await?;

        // Notify Discord of signup
        let _ = DiscordService::notify_user_signup(&email).await;

        Ok(user)
    }

    pub async fn start_email_verification_process(
        db_pool: &PgPool,
        user: &User,
    ) -> anyhow::Result<()> {
        let verification = VerificationRepository::insert(db_pool, &user.id).await?;

        // Send verification email
        EmailService::send_verification_email(user, &verification)
            .await
            .with_context(|| "Failed to send verification email")
    }

    pub async fn change_password(
        db_pool: &PgPool,
        user_id: &Uuid,
        new_password: String,
    ) -> anyhow::Result<User> {
        let password_hash = bcrypt::hash(new_password, bcrypt::DEFAULT_COST)
            .with_context(|| "Failed to hash password")?;
        let user = UserRepository::update_password(db_pool, user_id, &password_hash)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;
        Ok(user)
    }

    pub async fn delete_user(db_pool: &PgPool, user_id: &Uuid) -> anyhow::Result<()> {
        UserRepository::delete_from_id(db_pool, user_id)
            .await
            .with_context(|| "Failed to delete user")
    }
}
