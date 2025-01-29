use shared_data::{decode_response_v1, CollectorResponseV1, DATA_COLLECTION_ADDRESS};
use std::{collections::VecDeque, io::{Read, Write}};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CollectorError {
    #[error("unable to connect to the servers")]
    UnableToConnect,
    #[error("unable to write data to a stream")]
    UnableToSend,
    #[error("unable to receive data from a stream")]
    UnableToReceiveData,
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

    let mut resp_buf = vec![0u8;512];
    while let Some(cmd_b) = queue.pop_front() {
        if stream.write_all(&cmd_b).is_err() {
            queue.push_front(cmd_b);
            return Err(CollectorError::UnableToSend);
        }
        let bytes_read = stream.read(&mut resp_buf).map_err(|_| {CollectorError::UnableToReceiveData})?;
        if bytes_read == 0 {
            queue.push_front(cmd_b);
            return Err(CollectorError::UnableToReceiveData);
        };

        let ack = decode_response_v1(&resp_buf[0..bytes_read]);
        if ack != CollectorResponseV1::Ack(0) {
            queue.push_front(cmd_b);
            return Err(CollectorError::UnableToReceiveData);
        };
        println!("ack received");
    }
    Ok(())
}