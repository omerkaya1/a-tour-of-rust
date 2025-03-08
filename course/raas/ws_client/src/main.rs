use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut ws_stream, _) = tokio_tungstenite::connect_async("ws://127.0.0.1:3000/ws").await?;
    let message = Message::Binary("Hello".into());

    ws_stream.send(message).await?;

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        println!("received a message: {msg}");
    }

    Ok(())
}
