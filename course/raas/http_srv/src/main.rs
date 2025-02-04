use axum::{response::Html, routing::get, Router, extract::Path};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(handler))
    .route("/book/{id}", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

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