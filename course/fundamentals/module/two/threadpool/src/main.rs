use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>;

fn hi() {
    println!("Hello there!");
}

fn main() {
    let (tx, rx) = mpsc::channel::<Job>();
    let handle = std::thread::spawn(move || {
        while let Ok(job) = rx.recv() {
            job();
        }
    });

    let job = || println!("Hello from closure");
    let job2 = || {
        for i in 0..10 {
            println!("{i}");
        }
    };

    tx.send(Box::new(job)).unwrap();
    tx.send(Box::new(job2)).unwrap();
    tx.send(Box::new(hi)).unwrap();
    tx.send(Box::new(|| {
        for _ in 0..1 {
            println!("from the cycle")
        }
    })).unwrap();

    handle.join().unwrap();
    std::mem::drop(tx);
}
