#![allow(dead_code, unused)]

use shared_data::CollectorCommandV1;

mod collector;
mod sender;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<CollectorCommandV1>();

    // the collector thread initialisation
    let collector_thread = std::thread::spawn(move || {
        collector::collect_data(tx);
    });

    // send the received command over the wire
    while let Ok(cmd) = rx.recv() {
        let _ = sender::send_command(cmd); // error is ignored
    }
}