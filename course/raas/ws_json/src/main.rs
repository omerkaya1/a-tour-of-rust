use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/", get(home));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Html<&'static str> {
    const BODY: &str = include_str!("json.html");
    Html(BODY)
}

#[derive(Debug, Serialize, Deserialize)]
enum Request {
    NextPoint,
    Quit,
}

#[derive(Debug, Serialize, Deserialize)]
struct Point {
    x: f32,
    y: f32,
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |sock| handle_socket(sock))
}

async fn handle_socket(mut socket: WebSocket) {
    let mut tick = 0.0f32;

    while let Some(Ok(msg)) = socket.recv().await {
        let text = msg.to_text().unwrap();
        let req: Request = serde_json::from_str(text).unwrap();

        match req {
            Request::NextPoint => {
                let resp = Point {
                    x: tick.cos() * (tick / 10.0),
                    y: tick.sin() * (tick / 10.0),
                };
                let resp = serde_json::to_string(&resp).unwrap();
                socket.send(resp.into()).await.unwrap();

                tick += 0.01;
                if tick >= 400.0 {
                    tick = 0.0;
                }
            }
            Request::Quit => {
                break;
            }
        }
    }
}
