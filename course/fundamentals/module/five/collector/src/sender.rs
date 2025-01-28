use shared_data::DATA_COLLECTION_ADDRESS;
use std::{collections::VecDeque, io::Write};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CollectorError {
    #[error("unable to connect to the servers")]
    UnableToConnect,
    #[error("unable to write data to a stream")]
    UnableToSend,
}

// pub fn send_command(cmd_b: &[u8]) -> Result<(), CollectorError> {
//     let mut stream = std::net::TcpStream::connect(DATA_COLLECTION_ADDRESS)
//         .map_err(|_| CollectorError::UnableToConnect)?;
//     stream
//         .write_all(cmd_b)
//         .map_err(|_| CollectorError::UnableToSend)
// }

pub fn send_queue(queue: &mut VecDeque<Vec<u8>>) -> Result<(), CollectorError> {
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTION_ADDRESS).map_err(|_| {
        CollectorError::UnableToConnect
    })?;

    while let Some(cmd_b) = queue.pop_front() {
        if stream.write_all(&cmd_b).is_err() {
            queue.push_front(cmd_b);
            return Err(CollectorError::UnableToSend);
        }
    }
    Ok(())
}