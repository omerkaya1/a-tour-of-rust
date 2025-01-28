use std::collections::VecDeque;
use shared_data::{encode_v1, CollectorCommandV1};

mod collector;
mod sender;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<CollectorCommandV1>();

    let uid = collector::get_uuid();

    // the collector thread initialisation
    let collector_thread = std::thread::spawn(move || {
        collector::collect_data(uid, tx);
    });

    // send the received command over the wire
    let mut cmd_queue = VecDeque::with_capacity(120);
    while let Ok(cmd) = rx.recv() {
        let encoded = encode_v1(&cmd);
        cmd_queue.push_back(encoded);
        let _ = sender::send_queue(&mut cmd_queue); // error is ignored
    }

    collector_thread.join().unwrap();
}