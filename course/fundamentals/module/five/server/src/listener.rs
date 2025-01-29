use shared_data::{decode_v1, encode_response_v1, CollectorCommandV1, CollectorResponseV1, DATA_COLLECTION_ADDRESS};
use sqlx::{Pool, Sqlite};
use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

const DATA_INSERTION_QUERY: &str = "INSERT INTO timeseries (collector_id, received, total_memory, used_memory, average_cpu) VALUES ($1, $2, $3, $4, $5);";

async fn new_connection(mut socket: TcpStream, addr: SocketAddr, cnn: Pool<Sqlite>) {
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

        let (
            ts,
            CollectorCommandV1::SubmitData {
                collector_id,
                total_memory,
                used_memory,
                average_cpu_usage,
            },
        ) = received_data;
        let collector_id = uuid::Uuid::from_u128(collector_id);
        let collector_id_str = collector_id.to_string();

        let result = sqlx::query(DATA_INSERTION_QUERY)
            .bind(collector_id_str)
            .bind(ts)
            .bind(total_memory as i64)
            .bind(used_memory as i64)
            .bind(average_cpu_usage as i64)
            .execute(&cnn)
            .await;
        if result.is_err() {
            println!("failed to insert data into the database: {result:?}");
            continue;
        }
        let ack = CollectorResponseV1::Ack(0);
        let resp_bytes = encode_response_v1(ack);
        let _ = socket.write_all(&resp_bytes).await;
    }
}

pub async fn data_collector_listener(cnn: Pool<Sqlite>) -> anyhow::Result<()> {
    let listener = TcpListener::bind(DATA_COLLECTION_ADDRESS).await?;

    loop {
        let (socket, addr) = listener.accept().await?;
        tokio::spawn(new_connection(socket, addr, cnn.clone()));
    }
}
