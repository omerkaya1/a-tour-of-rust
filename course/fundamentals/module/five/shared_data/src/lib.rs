use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub const DATA_COLLECTION_ADDRESS: &str = "127.0.0.1:7001";
const MAGIC_NUMBER: u16 = 1234;
const VERSION_NUMBER: u16 = 1;

fn unix_now() -> u32 {
    let start = SystemTime::now();
    let since = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since.as_secs() as u32
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CollectorCommandV1 {
    SubmitData {
        collector_id: u128,
        total_memory: u64,
        used_memory: u64,
        average_cpu_usage: f32,
    },
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CollectorResponseV1 {
    Ack(u32),
}

pub fn encode_response_v1(resp: CollectorResponseV1) -> Vec<u8> {
    bincode::serialize(&resp).unwrap()
}

pub fn decode_response_v1(b: &[u8]) -> CollectorResponseV1 { // for proper error handling, we can do this: Result<CollectorResponseV1, Box<bincode::ErrorKind>>
    bincode::deserialize(b).unwrap()
}

pub fn encode_v1(cmd: &CollectorCommandV1) -> Vec<u8> {
    let payload_bytes = bincode::serialize(cmd).unwrap();
    let crc = crc32fast::hash(&payload_bytes);
    let payload_size = payload_bytes.len() as u32;
    let ts = unix_now();

    let mut result = Vec::with_capacity(140);
    result.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());
    result.extend_from_slice(&VERSION_NUMBER.to_be_bytes());
    result.extend_from_slice(&ts.to_be_bytes());
    result.extend_from_slice(&payload_size.to_be_bytes());
    result.extend_from_slice(&payload_bytes);
    result.extend_from_slice(&crc.to_be_bytes());
    result
}

pub fn decode_v1(bytes: &[u8]) -> (u32, CollectorCommandV1) {
    let magic_num = u16::from_be_bytes([bytes[0], bytes[1]]);
    let version = u16::from_be_bytes([bytes[2], bytes[3]]);
    let ts = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let payload_size = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
    let payload = &bytes[12..12 + payload_size as usize];
    let crc = u32::from_be_bytes([
        bytes[12 + payload_size as usize],
        bytes[13 + payload_size as usize],
        bytes[14 + payload_size as usize],
        bytes[15 + payload_size as usize],
    ]);

    // verify things
    assert_eq!(magic_num, MAGIC_NUMBER);
    assert_eq!(version, VERSION_NUMBER);
    let computed_crc = crc32fast::hash(payload);
    assert_eq!(crc, computed_crc);

    (ts, bincode::deserialize(payload).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_v1() {
        let cmd = CollectorCommandV1::SubmitData { 
            collector_id: 1234, 
            total_memory: 4321, 
            used_memory: 50, 
            average_cpu_usage: 0.5,
        };
        let encoded = encode_v1(&cmd);
        let (ts, decoded) = decode_v1(&encoded);
        assert_eq!(decoded, cmd);
        assert!(ts > 0)
    }

    #[test]
    fn test_encode_decode_response_v1() {
        let resp = CollectorResponseV1::Ack(123);
        let encoded = encode_response_v1(resp.clone());
        let decoded = decode_response_v1(&encoded);
        assert_eq!(decoded, resp);
    }
}
