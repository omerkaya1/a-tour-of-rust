use shared_data::{CollectorCommandV1, DATA_COLLECTION_ADDRESS};
use std::{io::Write, result};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CollectorError {
    #[error("unable to connect to the servers")]
    UnableToConnect,
    #[error("unable to write data to a stream")]
    UnableToSend,
}

pub fn send_command(cmd: CollectorCommandV1) -> Result<(), CollectorError> {
    let b = shared_data::encode_v1(cmd);
    println!("Encoded {} bytes", b.len());
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTION_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;
    stream
        .write_all(&b)
        .map_err(|_| CollectorError::UnableToSend)
}
