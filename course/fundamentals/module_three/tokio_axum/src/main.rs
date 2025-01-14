#![allow(dead_code)]

use axum::{response::Html, routing::get, Router};

async fn say_hello() -> Html<&'static str> {
    Html("<h1>Some hello in HTML</h1>")
}

async fn say_hi() -> &'static str {
    "HI!"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(say_hi))
        .route("/html_hi", get(say_hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
