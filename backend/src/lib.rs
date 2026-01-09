use crate::controllers::{UploadController, UserController};
use axum::Router;
use axum::{extract::State, http::StatusCode};
use sqlx::PgPool;
use sqlx::migrate::Migrator;
use std::path::Path;

pub mod controllers;
pub mod dtos;
pub mod entities;
pub mod repositories;
pub mod services;
pub mod utils;

pub fn app_router() -> Router<PgPool> {
    Router::new()
        .nest("/api/uploads", UploadController::router())
        .nest("/api/users", UserController::router())
}

pub async fn migrate_endpoint(State(db_pool): State<PgPool>) -> (StatusCode, String) {
    migrate(&db_pool).await
}

pub async fn migrate(db_pool: &PgPool) -> (StatusCode, String) {
    let migrations_path =
        env_var_with_default_on_empty("MIGRATIONS_PATH", "migrations".to_string());
    match Migrator::new(Path::new(&migrations_path)).await {
        Ok(migrator) => match migrator.run(db_pool).await {
            Ok(_) => {
                println!("Migrations applied successfully");
                (StatusCode::OK, "ok".to_string())
            },
            Err(e) => {
                println!("Failed to apply migrations, error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("failed to apply migrations, error: {e}"),
                )
            },
        },
        Err(e) => {
            println!("Failed to find migrations directory, error: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to find migrations directory, error: {e}"),
            )
        },
    }
}

pub fn get_db_url() -> String {
    match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            let postgres_user =
                env_var_with_default_on_empty("POSTGRES_USR", "postgres".to_string());
            let postgres_password = std::env::var("POSTGRES_PWD")
                .expect("A non-default password should be set with `POSTGRES_PWD` env var");
            let postgres_endpoint =
                env_var_with_default_on_empty("POSTGRES_EDP", "localhost".to_string());
            let postgres_port = env_var_with_default_on_empty("POSTGRES_PRT", "5432".to_string());
            let postgres_databasename =
                env_var_with_default_on_empty("POSTGRES_DBN", "postgres".to_string());
            let postgres_certificate = std::env::var("POSTGRES_CRT").unwrap_or("".to_string());

            if postgres_certificate.is_empty() {
                format!(
                    "postgres://{postgres_user}:{postgres_password}@{postgres_endpoint}:{postgres_port}/{postgres_databasename}"
                )
            } else {
                format!(
                    "postgres://{postgres_user}:{postgres_password}@{postgres_endpoint}:{postgres_port}/{postgres_databasename}?sslmode=verify-full&sslrootcert={postgres_certificate}"
                )
            }
        },
    }
}

pub fn env_var_with_default_on_empty(key: &str, default: String) -> String {
    match std::env::var(key) {
        Ok(var) => {
            if var.is_empty() {
                default
            } else {
                var
            }
        },
        Err(_) => default,
    }
}
