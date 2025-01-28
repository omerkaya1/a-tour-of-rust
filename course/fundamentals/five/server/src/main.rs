#![allow(dead_code)]

use axum::{routing::get, Extension, Router};
use tower_http::cors::CorsLayer;

mod handler;
mod listener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let db_url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::SqlitePool::connect(&db_url).await?;
    let handle = tokio::spawn(listener::data_collector_listener(pool.clone()));

    let app = Router::new()
        .route("/", get(handler::index))
        .route("/collector.html", get(handler::collector))
        .route("/api/all", get(handler::show_all))
        .route("/api/collector", get(handler::show_collectors))
        .route("/api/collector/{id}", get(handler::collector_data))
        .layer(CorsLayer::very_permissive())
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    handle.await??;
    Ok(())
}
