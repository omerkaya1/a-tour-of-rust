use tonic::{transport::Server, Request, Response, Status};

// this way we ensure that whatever resides in the `hello_module` is being pulled into
// the current scope under the specified name.
pub mod hello_module {
    tonic::include_proto!("hello");
}

use hello_module::greeter_server::{Greeter, GreeterServer};
use hello_module::{HelloRequest, HelloResponse};

use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    // NOTE: needs to be exactly the same name as it was defined in the proto file (the service section)
    #[tracing::instrument]
    async fn hello(&self, req: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        let resp = hello_module::HelloResponse {
            message: format!("hello {}", req.into_inner().name),
        };

        Ok(Response::new(resp))
    }
}

fn auth(req: Request<()>) -> Result<Request<()>, Status> {
    use tonic::metadata::MetadataValue;
    let token: MetadataValue<_> = "Bearer some-token".parse().unwrap();

    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("no valid token data")),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // add tracing subscriber
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_span_events(FmtSpan::CLOSE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .trace_fn(|_| tracing::info_span!("grpc-server"))
        .add_service(GreeterServer::with_interceptor(greeter, auth))
        .serve(addr)
        .await?;

    Ok(())
}
