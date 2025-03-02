use std::fmt::format;
use std::ptr::null;

use tonic::{transport::Server, Request, Response, Status};

// this way we ensure that whatever resides in the `hello_module` is being pulled into
// the current scope under the specified name.
pub mod hello_module {
    tonic::include_proto!("hello");
}

use hello_module::greeter_server::{Greeter, GreeterServer};
use hello_module::{HelloRequest, HelloResponse};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    // NOTE: needs to be exactly the same name as it was defined in the proto file (the service section)
    async fn hello(&self, req: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        let resp = hello_module::HelloResponse {
            message: format!("hello {}", req.into_inner().name),
        };

        Ok(Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
