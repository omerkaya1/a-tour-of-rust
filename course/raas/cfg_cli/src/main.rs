use axum::{response::Html, routing::get, Router};
use clap::{value_parser, Arg, Command};

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    if let Some(matches) = matches.subcommand_matches("serve") {
        let addr: String = matches
            .get_one("addr")
            .cloned()
            .unwrap_or("127.0.0.1".to_string());

        let port: u16 = *matches.get_one("port").unwrap_or(&3000);
        let bind_addr = format!("{}:{}", addr, port);

        server(&bind_addr).await;
        return;
    }
    panic!("unknown command received");
}

fn cli() -> Command {
    Command::new("http-server")
        .version("0.1.0")
        .author("omerkaya1")
        .subcommand(
            Command::new("serve")
                .about("")
                .arg(
                    Arg::new("addr")
                        .short('a')
                        .long("address")
                        .value_name("ADDRESS")
                        .help("set the IP address to bind to"),
                )
                .arg(
                    Arg::new("port")
                        .short('p')
                        .long("port")
                        .value_name("PORT")
                        .help("set the port to bind to")
                        .value_parser(value_parser!(u16)),
                ),
        )
}

async fn server(addr: &str) {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("addr: {}", addr);

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello!</h1>")
}
