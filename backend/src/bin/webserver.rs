use fileshare_backend::{app_router, get_db_url, migrate};
use sqlx::PgPool;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    env_logger::init();

    let axum_port =
        env::var("AXUM_PORT").map_or(3000, |e| e.parse().expect("AXUM_PORT should be a number"));

    let db_pool = PgPool::connect(&get_db_url())
        .await
        .expect("Connection to database should not fail");

    migrate(&db_pool).await;
    let app = app_router().with_state(db_pool);
    let listener = TcpListener::bind(format!("0.0.0.0:{axum_port}")).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
