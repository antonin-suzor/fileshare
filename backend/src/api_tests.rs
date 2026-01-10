use axum_test::TestServer;
use axum::http::StatusCode;
use chrono::{Days, Utc};
use serde_json::{Value, json};
use sqlx::PgPool;

fn app_test_server(db_pool: PgPool) -> TestServer {
    let app = crate::app_router().with_state(db_pool);
    TestServer::builder()
        .expect_success_by_default()
        .mock_transport()
        .build(app)
        .unwrap()
}

const BASIC_EMAIL: &'static str = "some@mail.com";
const BASIC_PASSWORD: &'static str = "pass";

async fn create_unverified_user_and_token(db_pool: &PgPool) -> (crate::entities::User, String) {
    let unverified_user = crate::repositories::UserRepository::create(
        db_pool,
        BASIC_EMAIL,
        &bcrypt::hash(BASIC_PASSWORD, bcrypt::DEFAULT_COST).unwrap(),
    )
    .await
    .unwrap();
    let token = crate::services::AuthService::create_jwt_for_user(&unverified_user).unwrap();
    (unverified_user, token)
}

async fn create_verified_user_and_token(db_pool: &PgPool) -> (crate::entities::User, String) {
    let unverified_user = crate::repositories::UserRepository::create(
        db_pool,
        BASIC_EMAIL,
        &bcrypt::hash(BASIC_PASSWORD, bcrypt::DEFAULT_COST).unwrap(),
    )
    .await
    .unwrap();
    let verification =
        crate::repositories::VerificationRepository::insert(db_pool, &unverified_user.id)
            .await
            .unwrap();
    let verified_user = crate::services::AuthService::verify(db_pool, verification.id)
        .await
        .unwrap();
    assert_eq!(verified_user.id, unverified_user.id);
    let token = crate::services::AuthService::create_jwt_for_user(&verified_user).unwrap();
    (verified_user, token)
}

#[sqlx::test]
async fn signup_gives_token_and_user(db_pool: PgPool) -> anyhow::Result<()> {
    let server = app_test_server(db_pool);

    let response = server
        .post("/api/users/signup")
        .json(&json!({"email": BASIC_EMAIL, "password": BASIC_PASSWORD}))
        .expect_success()
        .await;
    let body: Value = response.json();

    assert!(body["token"].is_string());
    assert!(body["user"].is_object());

    Ok(())
}

#[sqlx::test]
async fn login_gives_token_and_user(db_pool: PgPool) -> anyhow::Result<()> {
    let server = app_test_server(db_pool);

    server
        .post("/api/users/signup")
        .json(&json!({"email": BASIC_EMAIL, "password": BASIC_PASSWORD}))
        .expect_success()
        .await;

    let response = server
        .post("/api/users/login")
        .json(&json!({"email": BASIC_EMAIL, "password": BASIC_PASSWORD}))
        .expect_success()
        .await;
    let body: Value = response.json();

    assert!(body["token"].is_string());
    assert!(body["user"].is_object());

    Ok(())
}

#[sqlx::test]
async fn cannot_signup_with_existing_email(db_pool: PgPool) -> anyhow::Result<()> {
    create_unverified_user_and_token(&db_pool).await;
    let server = app_test_server(db_pool);

    server
        .post("/api/users/signup")
        .json(&json!({"email": BASIC_EMAIL, "password": BASIC_PASSWORD}))
        .expect_failure()
        .await;

    Ok(())
}

#[sqlx::test]
async fn cannot_start_upload_without_token(db_pool: PgPool) -> anyhow::Result<()> {
    create_unverified_user_and_token(&db_pool).await;
    let server = app_test_server(db_pool);

    let res = server
        .post("/api/uploads/start")
        .json(&json!({"file_name": "file.txt", "expires_at": Utc::now().checked_add_days(Days::new(1)).unwrap().to_rfc3339(), "content_type": "text/plain"}))
        .expect_failure()
        .await;
    assert_eq!(res.status_code(), StatusCode::UNAUTHORIZED);

    Ok(())
}

#[sqlx::test]
async fn cannot_start_upload_with_unverified_user(db_pool: PgPool) -> anyhow::Result<()> {
    let (_, token) = create_unverified_user_and_token(&db_pool).await;
    let server = app_test_server(db_pool);

    let res = server
        .post("/api/uploads/start")
        .authorization_bearer(token)
        .json(&json!({"file_name": "file.txt", "expires_at": Utc::now().checked_add_days(Days::new(1)).unwrap().to_rfc3339(), "content_type": "text/plain"}))
        .expect_failure()
        .await;
    assert_eq!(res.status_code(), StatusCode::FORBIDDEN);

    Ok(())
}

#[sqlx::test]
async fn can_start_upload_with_verified_user(db_pool: PgPool) -> anyhow::Result<()> {
    let (_, token) = create_verified_user_and_token(&db_pool).await;
    let server = app_test_server(db_pool);

    let response = server
        .post("/api/uploads/start")
        .authorization_bearer(token)
        .json(&json!({"file_name": "file.txt", "expires_at": Utc::now().checked_add_days(Days::new(1)).unwrap().to_rfc3339(), "content_type": "text/plain"}))
        .expect_success()
        .await;

    let body: Value = response.json();
    assert!(body["url"].is_string());

    Ok(())
}
