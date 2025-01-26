#![allow(dead_code)]

mod listener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let handle = tokio::spawn(listener::data_collector_listener());

    handle.await??;
    Ok(())
}
