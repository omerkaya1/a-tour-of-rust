fn hello_thread(n: u32) {
    println!("hello from thread {n}");
}

fn do_math(i: u32) -> u32 {
    let mut n = i+1;

    for _ in 0..10 {
        n *= 2;
    }
    n
}

fn main() {
    println!("hello from the main thread");

    let mut thread_handlers = Vec::new();
    for i in 0..5 {
        // the move is here to pass the copy of i to the hello_thread func
        let thread_handle = std::thread::spawn(move || do_math(i));
        thread_handlers.push(thread_handle);
    }

    thread_handlers.into_iter().for_each(|h|
        println!("{}", h.join().unwrap())
    );
}
