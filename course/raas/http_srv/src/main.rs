use std::collections::HashMap;

use axum::{extract::Path, extract::Query, response::Html, routing::get, Router};
use axum::http::HeaderMap;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/book/{id}", get(path_extract))
        .route("/book", get(query_extract))
        .route("/header", get(header_extract));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello!</h1>")
}

// path extraction logic
async fn path_extract(Path(id): Path<u32>) -> Html<String> {
    println!("{id}");
    Html(format!("id passed: {id}"))
}

// query extraction logic
async fn query_extract(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    Html(format!("query params passed: {params:#?}"))
}

// header extraction logic
async fn header_extract(headers: HeaderMap) -> Html<String> {
    Html(format!("headers passed: {headers:#?}"))
}
