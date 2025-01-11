use std::{
    time::Duration,
    sync::mpsc
};

enum Command {
    Print(String)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel::<Command>();
    let (tx_resp, mut rx_resp) = tokio::sync::mpsc::channel::<String>(10);

    let handle = tokio::runtime::Handle::current();

    std::thread::spawn(move || {
        while let Ok(cmd) = rx.recv() {
            match cmd {
                Command::Print(s) => {
                    let tx_resp = tx_resp.clone();

                    handle.spawn(async move {
                        tx_resp.send(s).await.unwrap();
                    });

                    // println!("{s}")
                },
            }
        }
    });

    // receive data from a channel
    tokio::spawn(async move {
        while let Some(resp) = rx_resp.recv().await {
            println!("{resp}");
        }
    });

    let mut counter = 0;
    loop {
        tokio::time::sleep(Duration::from_millis(100)).await;
        tx.send(Command::Print(format!("counter: {counter}"))).unwrap();
        counter += 1;
    }

    // broadcast channel - alike sync.Cond?
    // let (tx, mut rx) = tokio::sync::broadcast::channel::<String>(10);

    // tx.send("beep".to_string());
}
