use crate::{
    dtos::{
        ChangePasswordRequest, LoginRequest, LoginResponse, SignUpRequest, SignUpResponse,
        UserResponse, VerifyResponse,
    },
    services::{AuthService, UserService},
    utils::{ApiMessage, context_to_500},
};
use anyhow::Context;
use axum::{
    Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::{get, patch, post},
};
use sqlx::PgPool;
use uuid::Uuid;

/// Controller for /api/users
pub struct UserController {}
impl UserController {
    /// GET /api/users/me
    pub async fn get_api_users_me(
        State(db_pool): State<PgPool>,
        headers: HeaderMap,
    ) -> Result<Json<UserResponse>, ApiMessage> {
        let user = AuthService::get_user_from_auth_header(&db_pool, &headers).await?;
        Ok(Json(user.into()))
    }

    /// POST /api/users/signup
    pub async fn post_api_users_signup(
        State(db_pool): State<PgPool>,
        Json(request): Json<SignUpRequest>,
    ) -> Result<Json<SignUpResponse>, ApiMessage> {
        let user_db = UserService::signup(&db_pool, &request.email, &request.password)
            .await
            .with_context(|| "Failed to signup user")
            .map_err(context_to_500)?;
        Ok(Json(SignUpResponse {
            token: AuthService::create_jwt_for_user(&user_db)
                .with_context(|| "Failed to create JWT")
                .map_err(context_to_500)?,
            user: user_db.into(),
        }))
    }

    /// POST /api/users/login
    pub async fn post_api_users_login(
        State(db_pool): State<PgPool>,
        Json(request): Json<LoginRequest>,
    ) -> Result<Json<LoginResponse>, ApiMessage> {
        match UserService::from_email(&db_pool, &request.email)
            .await
            .with_context(|| "Failed to get user from email")
            .map_err(context_to_500)?
        {
            Some(user_db) => {
                if bcrypt::verify(request.password, &user_db.password_hash)
                    .with_context(|| "Failed to verify password")
                    .map_err(context_to_500)?
                {
                    Ok(Json(LoginResponse {
                        token: AuthService::create_jwt_for_user(&user_db)
                            .with_context(|| "Failed to create JWT")
                            .map_err(context_to_500)?,
                        user: user_db.into(),
                    }))
                } else {
                    Err(ApiMessage {
                        status: StatusCode::UNAUTHORIZED,
                        message: "Wrong password".to_string(),
                    })
                }
            },
            None => Err(ApiMessage {
                status: StatusCode::UNAUTHORIZED,
                message: "Wrong email".to_string(),
            }),
        }
    }

    /// POST /api/users/verify/{verification_id}
    pub async fn post_api_users_verify_verification_id(
        State(db_pool): State<PgPool>,
        Path(verification_id): Path<Uuid>,
    ) -> Result<Json<VerifyResponse>, ApiMessage> {
        let user = AuthService::verify(&db_pool, verification_id)
            .await
            .with_context(|| "Failed to verify user")
            .map_err(context_to_500)?;
        Ok(Json(VerifyResponse {
            token: AuthService::create_jwt_for_user(&user)
                .with_context(|| "Failed to create JWT")
                .map_err(context_to_500)?,
            user: user.into(),
        }))
    }

    /// POST /api/users/me/send-verification
    pub async fn post_api_users_me_send_verification(
        State(db_pool): State<PgPool>,
        headers: HeaderMap,
    ) -> Result<(), ApiMessage> {
        let user = AuthService::get_user_from_auth_header(&db_pool, &headers).await?;
        UserService::start_email_verification_process(&db_pool, &user)
            .await
            .with_context(|| "Failed to send verification email")
            .map_err(context_to_500)?;
        Ok(())
    }

    /// PATCH /api/users/me/password
    pub async fn patch_api_users_me_password(
        State(db_pool): State<PgPool>,
        headers: HeaderMap,
        Json(request): Json<ChangePasswordRequest>,
    ) -> Result<StatusCode, ApiMessage> {
        let user = AuthService::get_user_from_auth_header(&db_pool, &headers).await?;

        UserService::change_password(&db_pool, &user.id, request.password)
            .await
            .with_context(|| "Failed to change password")
            .map_err(context_to_500)?;

        Ok(StatusCode::NO_CONTENT)
    }

    /// DELETE /api/users/me
    pub async fn delete_api_users_me(
        State(db_pool): State<PgPool>,
        headers: HeaderMap,
    ) -> Result<StatusCode, ApiMessage> {
        let user = AuthService::get_user_from_auth_header(&db_pool, &headers).await?;

        UserService::delete_user(&db_pool, &user.id)
            .await
            .with_context(|| "Failed to delete user")
            .map_err(context_to_500)?;

        Ok(StatusCode::NO_CONTENT)
    }

    /// Router to nest in /api/users
    pub fn router() -> Router<PgPool> {
        Router::new()
            .route("/login", post(Self::post_api_users_login))
            .route(
                "/me",
                get(Self::get_api_users_me).delete(Self::delete_api_users_me),
            )
            .route(
                "/me/send-verification",
                post(Self::post_api_users_me_send_verification),
            )
            .route("/me/password", patch(Self::patch_api_users_me_password))
            .route("/signup", post(Self::post_api_users_signup))
            .route(
                "/verify/{verification_id}",
                post(Self::post_api_users_verify_verification_id),
            )
    }
}
