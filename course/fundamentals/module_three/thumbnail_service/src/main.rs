#![allow(dead_code)]

use axum::{response::Html, extract::Multipart, routing::{get, post}, Extension, Router};
use sqlx::Row;

async fn test(Extension(pool): Extension<sqlx::SqlitePool>) -> String {
    let result = sqlx::query("SELECT COUNT(id) FROM images")
        .fetch_one(&pool)
        .await
        .unwrap();
    let count = result.get::<i64, _>(0);
    format!("{count} images in the database")
}

async fn index_page() -> Html<String> {
    let path = std::path::Path::new("src/index.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

async fn uploader(mut multipart: Multipart) -> String {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("{name} is {} bytes", data.len());
    }
    "Ok".to_string()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;

    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    // Build Axum with an "extension" to hold the database connection pool
    let app = Router::new()
    .route("/", get(test))
    .route("/index", get(index_page))
    .route("/upload", post(uploader))
    .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
