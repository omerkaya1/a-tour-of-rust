pub mod streaming {
	tonic::include_proto!("streaming");
}

use streaming::{streaming_client::StreamingClient, Start};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let mut client = StreamingClient::connect("http://[::1]:50051").await?;

	println!("requesting squares...");
	for n in 1..10 {
		println!("requesting squares up to {n}");
		let req = tonic::Request::new(Start{n});

		let mut stream = client.squares(req).await.unwrap().into_inner();

		while let Some(result) = stream.message().await.unwrap() {
			println!("result={:?}", result);
		}
	}
	Ok(())
} 