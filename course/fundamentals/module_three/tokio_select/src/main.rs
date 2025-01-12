use std::time::Duration;
use tokio::sync::{
    broadcast,
    mpsc,
};

// tokio::select is implemented through a macro

async fn do_cool() {
    tokio::time::sleep(Duration::from_millis(100)).await;
}

async fn timeout(dur: u64) {
    tokio::time::sleep(Duration::from_millis(dur)).await;
}

async fn receiver(mut rx: mpsc::Receiver<u32>, mut brx: broadcast::Receiver<u32>) {
    loop {
        tokio::select! {
            Some(n) = rx.recv() => println!("received a message {n} on the mpsc channel"),
            Ok(n) = brx.recv() => println!("received a message {n} on the broadcast channel"),
        }
    }
}

#[tokio::main]
async fn main() {
    // simple use of select through timeouts in async funcs
    tokio::select! {
        _ = do_cool() => println!("do_cool finished"),
        _ = timeout(50) => println!("timeout finished"),
    }

    // a slightly more complex example
    let (tx, rx) = mpsc::channel::<u32>(1);
    let (btx, brx) = broadcast::channel::<u32>(1);

    tokio::spawn(receiver(rx, brx));

    for i in 0..10 {
        if i % 2 == 0 {
            tx.send(i).await.unwrap();
            continue;
        } 
        btx.send(i).unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
