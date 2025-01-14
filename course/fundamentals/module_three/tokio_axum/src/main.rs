#![allow(dead_code)]

use axum::{response::Html, routing::get, Router};
use serde::Serialize;

async fn say_hello() -> Html<&'static str> {
    Html("<h1>Some hello in HTML</h1>")
}

async fn say_hi() -> &'static str {
    "HI!"
}

#[derive(Serialize)]
struct HelloJSON {
    message: String
}

async fn hello_json() -> axum::Json<HelloJSON> {
    // axum::Json::from(...) also works
    axum::Json(HelloJSON{message:"some data".to_string()})
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(say_hi))
        .route("/html_hi", get(say_hello))
        .route("/json_hi", get(hello_json));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
