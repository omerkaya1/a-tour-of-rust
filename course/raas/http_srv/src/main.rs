use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering::Relaxed},
    Arc,
};

use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{extract::Path, extract::Query, response::Html, routing::get, Router};
use axum::{Extension, Json};
use reqwest::StatusCode;
use tower_http::services::ServeDir;

struct MyStruct {
    text: String,
}

struct Counter {
    cnt: AtomicUsize,
}

struct MyState(i32);

fn service_one() -> Router {
    let state = Arc::new(MyState(5));
    Router::new()
        .route("/", get(service_one_handler))
        .with_state(state)
}

async fn service_one_handler(
    Extension(cnt): Extension<Arc<Counter>>,
    State(state): State<Arc<MyState>>,
) -> Html<String> {
    cnt.cnt.fetch_add(1, Relaxed);
    Html(format!("Service {} - {}", cnt.cnt.load(Relaxed), state.0))
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
        .route("/check", get(req_handler))
        .route("/status", get(status))
        .route("/time", get(handler_time))
        .route("/static", get(static_handler))
        .route("/request-id", get(header_handler))
        .fallback_service(ServeDir::new("web"))
        .layer(Extension(shared_counter))
        .layer(Extension(shared_text));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tokio::spawn(request_with_id());

    axum::serve(listener, app).await.unwrap();
}

async fn handler(
    Extension(cnt): Extension<Arc<Counter>>,
    Extension(text): Extension<Arc<MyStruct>>,
) -> Json<usize> {
    cnt.cnt.fetch_add(1, Relaxed);
    Json(cnt.cnt.load(Relaxed))
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

async fn req_handler() -> Html<String> {
    println!("sending a GET request");
    let cur_cnt = reqwest::get("http://127.0.0.1/")
        .await
        .unwrap()
        .json::<i32>()
        .await
        .unwrap();
    Html(format!("<h1>Remote counter: {cur_cnt}</h1>"))
}

async fn status() -> Result<impl IntoResponse, reqwest::StatusCode> {
    if true {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(Json(42))
}

async fn handler_time() -> Result<impl IntoResponse, (reqwest::StatusCode, String)> {
    let start = std::time::SystemTime::now();

    let secs = start
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "bad clock".to_string()))?
        .as_secs()
        % 3;

    let devided = 100u64
        .checked_div(secs)
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "div by zero".to_string()))?;

    Ok(Json(devided))
}

async fn static_handler() -> Result<impl IntoResponse, StatusCode> {
    Ok(Html("<h1>static handler</h1>"))
}

async fn header_handler(headers: HeaderMap) -> Html<String> {
    if let Some(h) = headers.get("x-request-id") {
        return Html(format!("request id: {}", h.to_str().unwrap()));
    }
    Html("x-request-id header not found".to_string())
}

async fn request_with_id() {
    tokio::time::sleep(std::time::Duration::from_millis(1500)).await;

    let resp = reqwest::Client::new()
        .get("http://127.0.0.1:3000/request-id")
        .header("x-request-id", "1234")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{}", resp);
}
