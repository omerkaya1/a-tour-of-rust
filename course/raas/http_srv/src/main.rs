use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering::Relaxed},
    Arc,
};

use axum::http::HeaderMap;
use axum::Extension;
use axum::{extract::Path, extract::Query, response::Html, routing::get, Router};

struct MyStruct {
    text: String,
}

struct Counter {
    cnt: AtomicUsize,
}

fn service_one() -> Router {
    Router::new().route("/", get(|| async {
        Html("service one".to_string())
    }))
}

#[tokio::main]
async fn main() {
    let shared_counter = Arc::new(Counter {
        cnt: AtomicUsize::new(0),
    });

    let shared_text = Arc::new(MyStruct {
        text: "some".to_string(),
    });

    let app = Router::new()
        .nest("/1", service_one()) // sub routing
        .route("/", get(handler))
        .route("/book/{id}", get(path_extract))
        .route("/book", get(query_extract))
        .route("/header", get(header_extract))
        .layer(Extension(shared_counter))
        .layer(Extension(shared_text));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn handler(
    Extension(cnt): Extension<Arc<Counter>>,
    Extension(text): Extension<Arc<MyStruct>>,
) -> Html<String> {
    cnt.cnt.fetch_add(1, Relaxed);
    Html(format!(
        "<h1>{} You are the visitor #{}</h1>",
        text.text,
        cnt.cnt.load(Relaxed)
    ))
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
