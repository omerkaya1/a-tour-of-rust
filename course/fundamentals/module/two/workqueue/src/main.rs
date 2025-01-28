use std::{collections::VecDeque, sync::Mutex, time::Duration};

use once_cell::sync::Lazy;

static WORK_QUEUE: Lazy<Mutex<VecDeque<String>>> = Lazy::new(|| Mutex::new(VecDeque::new()));

fn main() {
    let cpu_count = 2;
    let mut threads = Vec::with_capacity(cpu_count);
    let mut broadcast = Vec::with_capacity(cpu_count);

    for cpu in 0..cpu_count {
        let (tx, rx) = std::sync::mpsc::channel::<()>();

        broadcast.push(tx);

        let thread = std::thread::spawn(move || {
            while rx.recv().is_ok() {
                let mut lock = WORK_QUEUE.lock().unwrap();
                if let Some(lock) = lock.pop_front() {
                    std::mem::drop(lock);
                    println!("CPU {cpu} got a job to process");
                    std::thread::sleep(Duration::from_millis(500));
                    println!("CPU {cpu} finished working");
                    continue;
                }
                println!("CPU {cpu} is idle")
            }
        });

        threads.push(thread);
    }

    loop {
        let sent = {
            let mut lock = WORK_QUEUE.lock().unwrap();
            let n = lock.len();

            println!("{n} items in the queue");
            if n < 5 {
                lock.push_back("BEEP!".to_string());
                true
            } else {
                false
            }
        };

        if sent {
            broadcast.iter().for_each(|tx| {
                tx.send(()).unwrap();
            });
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
