use std::{net::SocketAddr, str::FromStr};

use tokio::io::BufReader;
use pin_project_lite::pin_project;
use axum::{body::StreamBody, http::{header, HeaderMap, StatusCode}, response::IntoResponse, routing::get, Router};


#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let addr = SocketAddr::from_str("0.0.0.0:3000").unwrap();

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    use tokio::io::AsyncBufReadExt;

    let file = match tokio::fs::File::open("Cargo.toml").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("file not found: {}", err))),
    };

    let stream = BufReader::new(file).lines();
    let stream = tokio_stream::wrappers::LinesStream::new(stream);
    let stream = ToUpper::new(stream);

    let body = StreamBody::from(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
            header::CONTENT_TYPE, 
            header::HeaderValue::from_static("text/toml; charset=utf-8"),
        );
    headers.insert(
        header::CONTENT_DISPOSITION, 
        header::HeaderValue::from_str("attachment; filename=\"Cargo.toml\"").unwrap(),
    );

    Ok((headers, body))
}

pin_project! {
    struct ToUpper {
        #[pin]
        stream: tokio_stream::wrappers::LinesStream<BufReader<tokio::fs::File>>,
    }
}

impl ToUpper {
    fn new(stream: tokio_stream::wrappers::LinesStream<BufReader<tokio::fs::File>>) -> Self {
        Self { stream }
    }
}

impl tokio_stream::Stream for ToUpper {
    type Item = std::io::Result<String>;

    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        self.project().stream.poll_next(cx).map(|opt| {
            opt.map(|res| {
                res.map(|line| {
                    line.to_uppercase() + "\n"
                })
            })
        })
    }
}