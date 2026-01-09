use fileshare_backend::{app_router, get_db_url, migrate};
use lambda_http::{Error, run, tracing};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    let _ = dotenvy::dotenv();
    env_logger::init(); // useless for now

    let db_pool = PgPool::connect(&get_db_url())
        .await
        .expect("Connection to database should not fail");

    migrate(&db_pool).await;
    let app = app_router().with_state(db_pool);
    run(app).await
}
