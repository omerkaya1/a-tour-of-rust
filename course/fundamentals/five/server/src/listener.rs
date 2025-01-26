use shared_data::{decode_v1, DATA_COLLECTION_ADDRESS};
use std::net::SocketAddr;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

async fn new_connection(mut socket: TcpStream, addr: SocketAddr) {
    println!("receiving new connection from {addr:?}");
    let mut buf = vec![0u8; 1024];

    loop {
        let n = socket
            .read(&mut buf)
            .await
            .expect("failed to read data from socket");

		if n == 0 {
			println!("no data received");
			return;
		}

		println!("received {n} bytes");  
		let received_data = decode_v1(&buf[0..n]);
		println!("received data {received_data:?}");
    }
}

pub async fn data_collector_listener() -> anyhow::Result<()> {
    let listener = TcpListener::bind(DATA_COLLECTION_ADDRESS).await?;

    loop {
        let (socket, addr) = listener.accept().await?;
        tokio::spawn(new_connection(socket, addr));
    }
}
