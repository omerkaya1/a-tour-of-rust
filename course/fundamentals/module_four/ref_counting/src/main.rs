use std::sync::Arc;

#[derive(Debug)]
struct Droppable(i32);

impl Droppable {
    fn new(n: i32) -> Self {
        println!("Constructing {n}");
        Self(n)
    }
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn move_me(x: Arc<Droppable>) {
    println!("Moved {}", x.0);
}

struct SharedData(String);

fn main() {
    // let my_shared = Arc::new(Droppable::new(1));
    // {
    //     let _x = my_shared.clone();
    //     let _y = my_shared.clone();
    //     let _z = my_shared.clone();
    // }
    // move_me(my_shared.clone());

    // let mut threads = Vec::new();
    // for _ in 0..10 {
    //     let my_clone = my_shared.clone();
    //     threads.push(std::thread::spawn(move || {
    //         println!("{my_clone:?}");
    //     }));
    // }

    // for t in threads {
    //     t.join().unwrap();
    // }

    // println!("{my_shared:?}");
    // println!("Application exit");

    use std::sync::{Arc, Mutex};

    let my_shared = Arc::new(Mutex::new(SharedData("Hello".to_string())));
    let mut threads = Vec::new();
    for i in 0..10 {
        let my_shared = my_shared.clone();
        threads.push(std::thread::spawn(move || {
            let mut data = my_shared.lock().unwrap();
            data.0.push_str(&format!(" {i}"));
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    let data = my_shared.lock().unwrap();
    println!("{}", data.0);
}