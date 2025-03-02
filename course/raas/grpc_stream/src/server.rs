use tonic::{transport::Server, Request, Response, Status};
use tokio_stream::wrappers::ReceiverStream;

pub mod streaming {
    tonic::include_proto!("streaming");
}

use streaming::streaming_server::{Streaming, StreamingServer};
use streaming::{Start, Square};

#[derive(Debug, Default)]
pub struct StreamingService {}

#[tonic::async_trait]
impl Streaming for StreamingService {
    type SquaresStream = ReceiverStream<Result<Square, Status>>;

    async fn squares(&self, req: Request<Start>) -> Result<Response<Self::SquaresStream>, Status> {
        let (tx, rx) = tokio::sync::mpsc::channel(4);

        tokio::spawn(async move {
            for i in 0..req.into_inner().n {
                let square = Square{
                    n: i * i,
                };
                tx.send(Ok(square)).await.unwrap();
            };
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse()?;
    let streamer_srv = StreamingService::default();

    Server::builder()
        .add_service(StreamingServer::new(streamer_srv))
        .serve(addr)
        .await?;

    Ok(())
}
