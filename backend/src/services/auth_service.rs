use crate::{
    entities::User,
    repositories::{UserRepository, VerificationRepository},
    services::DiscordService,
    utils::ApiMessage,
};
use axum::http::{HeaderMap, StatusCode};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

pub struct AuthService {}
impl AuthService {
    pub async fn get_user_from_auth_header(
        db_pool: &PgPool,
        headers: &HeaderMap,
    ) -> Result<User, ApiMessage> {
        let auth_header = headers
            .get("Authorization")
            .ok_or(ApiMessage {
                status: StatusCode::UNAUTHORIZED,
                message: "Missing authorization header".to_string(),
            })?
            .to_str()
            .map_err(|_| ApiMessage {
                status: StatusCode::UNAUTHORIZED,
                message: "Invalid authorization header".to_string(),
            })?;
        if !auth_header.starts_with("Bearer ") {
            return Err(ApiMessage {
                status: StatusCode::UNAUTHORIZED,
                message: "Invalid authorization header".to_string(),
            });
        }
        let token = auth_header.trim_start_matches("Bearer ").trim();

        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| ApiMessage {
            status: StatusCode::UNAUTHORIZED,
            message: "Invalid token".to_string(),
        })?;
        let user_id = Uuid::parse_str(&token_data.claims.sub).map_err(|_| ApiMessage {
            status: StatusCode::UNAUTHORIZED,
            message: "Invalid token subject".to_string(),
        })?;

        UserRepository::from_id(db_pool, &user_id)
            .await
            .map_err(|_| ApiMessage {
                status: StatusCode::UNAUTHORIZED,
                message: "Invalid token subject".to_string(),
            })?
            .ok_or(ApiMessage {
                status: StatusCode::UNAUTHORIZED,
                message: "Server-side error when authorizing token".to_string(),
            })
    }

    pub fn create_jwt_for_user(user: &User) -> anyhow::Result<String> {
        let secret = env::var("JWT_SECRET").expect("env var JWT_SECRET should be set");
        let expiration = Utc::now().timestamp() + (60 * 60 * 24 * 7); // 7 days
        let claims = Claims {
            sub: user.id.to_string(),
            exp: expiration as usize,
        };
        Ok(jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )?)
    }

    pub async fn verify(db_pool: &PgPool, verification_id: Uuid) -> anyhow::Result<User> {
        let verification = VerificationRepository::from_id(db_pool, &verification_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Verification not found"))?;
        if verification.activated_at.is_some() {
            return Err(anyhow::anyhow!("This verification has already been done"));
        }
        match verification.user_id {
            Some(user_id) => {
                VerificationRepository::set_activated_at_now(db_pool, &verification_id).await?;
                let user =
                    UserRepository::set_user_verification(db_pool, &user_id, &verification_id)
                        .await?
                        .ok_or_else(|| anyhow::anyhow!("User not found"))?;

                // Notify Discord of email verification
                let _ = DiscordService::notify_email_verified(&user.email).await;

                Ok(user)
            },
            None => Err(anyhow::anyhow!("No user for that verification")),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
