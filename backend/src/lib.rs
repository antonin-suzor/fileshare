use crate::controllers::{UploadController, UserController};
use axum::Router;
use axum::http::StatusCode;
use sqlx::PgPool;
use sqlx::migrate::Migrator;
use std::{env, path::Path};

#[cfg(test)]
mod api_tests;

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

pub async fn migrate(db_pool: &PgPool) -> (StatusCode, String) {
    let migrations_path = env::var("MIGRATIONS_PATH").unwrap_or("migrations".to_string());
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
