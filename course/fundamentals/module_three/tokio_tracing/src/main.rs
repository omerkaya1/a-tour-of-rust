#![allow(dead_code)]

use std::time::Duration;

#[tracing::instrument]
async fn hello() {
    println!("hello!");
    tokio::time::sleep(Duration::from_millis(100)).await;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tracing_subscriber::fmt::format::FmtSpan; // this can be substituted easily with opentelemetry

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT | FmtSpan::CLOSE)
        // enable if you need thhe messages to be JSON serialised
        // .json()
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!("starting up");

    tracing::warn!("this is a warning message");

    tracing::error!("some error");

    hello().await;

    Ok(())
}
