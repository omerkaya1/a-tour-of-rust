use std::thread;

fn my_thread() {
    println!(
        "Hello from a thread named {}",
        thread::current().name().unwrap()
    );
}

fn main() {
    let h = thread::Builder::new()
        .name("named_thread".to_string())
        .stack_size(std::mem::size_of::<usize>() * 4)
        .spawn(my_thread)
        .unwrap();

    h.join().unwrap();
}
