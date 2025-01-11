use tokio::sync::Mutex;
use once_cell::sync::Lazy;

static COUNTER: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

async fn add_one(n: u32) -> u32 {
    n + 1
}

async fn inc() {
    let mut counter = COUNTER.lock().await;
    *counter = add_one(*counter).await;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tokio::join!(inc(), inc(), inc());
    println!("Counter: {}", *COUNTER.lock().await);
    Ok(())
}
