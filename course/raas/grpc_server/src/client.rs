pub mod hello_module {
    tonic::include_proto!("hello");
}

use hello_module::greeter_client::GreeterClient;
use hello_module::HelloRequest;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tonic::Request;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let channel = Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;

    let token: MetadataValue<_> = "Bearer some-token".parse()?;

    let mut client = GreeterClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    let req = tonic::Request::new(HelloRequest{
        name: "Client".into(),
    });

    let resp = client.hello(req).await?;

    println!("Response: {:?}", resp);

    Ok(())
}
