fn hello_thread(n: u32) {
    println!("hello from thread {n}");
}

fn main() {
    println!("hello from the main thread");

    let mut thread_handlers = Vec::new();
    for i in 0..5 {
        // the move is here is to pass the copy of i to the hello_thread func
        let thread_handle = std::thread::spawn(move || hello_thread(i));
        thread_handlers.push(thread_handle);
    }

    thread_handlers.into_iter().for_each(|h| h.join().unwrap());
}
