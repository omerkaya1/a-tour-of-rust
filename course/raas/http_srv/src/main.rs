use std::collections::HashMap;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering::Relaxed}};

use axum::extract::State;
use axum::http::HeaderMap;
use axum::{extract::Path, extract::Query, response::Html, routing::get, Router};

struct MyStruct {
    cfg: String,
    cnt: AtomicUsize,
}

#[tokio::main]
async fn main() {
    let shared_cfg = Arc::new(MyStruct{
        cfg: "some_str".to_string(),
        cnt: AtomicUsize::new(0),
    });

    let app = Router::new()
        .route("/", get(handler))
        .route("/book/{id}", get(path_extract))
        .route("/book", get(query_extract))
        .route("/header", get(header_extract))
        .with_state(shared_cfg);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn handler(
    State(cfg): State<Arc<MyStruct>>
) -> Html<String> {
    cfg.cnt.fetch_add(1, Relaxed);
    Html(format!("<h1>{} - counter = {}</h1>", cfg.cfg, cfg.cnt.load(Relaxed)))
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
