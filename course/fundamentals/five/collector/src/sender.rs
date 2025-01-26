use shared_data::{
	CollectorCommandV1,
	DATA_COLLECTION_ADDRESS,
};
use std::{
	io::Write, result,
};

pub fn send_command(cmd: CollectorCommandV1) {
	let b = shared_data::encode_v1(cmd);
	println!("Encoded {} bytes", b.len());
	let mut stream_result = std::net::TcpStream::connect(DATA_COLLECTION_ADDRESS);
	match stream_result {
		Ok(mut s) => s.write_all(&b).unwrap(),
		Err(e) => println!("failed to establich a tcp connection: {e:?}"),
	};
}