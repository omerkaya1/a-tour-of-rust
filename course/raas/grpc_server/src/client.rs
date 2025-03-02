pub mod hello_module {
    tonic::include_proto!("hello");
}

use hello_module::greeter_client::GreeterClient;
use hello_module::HelloRequest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let req = tonic::Request::new(HelloRequest{
        name: "Client".into(),
    });

    let resp = client.hello(req).await?;

    println!("Response: {:?}", resp);

    Ok(())
}
