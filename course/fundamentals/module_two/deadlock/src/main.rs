use std::sync::Mutex;

static SHARED: Mutex<u32> = Mutex::new(0);

fn main() {

    let lock = SHARED.lock().unwrap();
    // uncommenting this line will lead to a deadlock
    // putting a scope around the lock is sufficient to avoid deadlocks
    // example:
    // {
    //    let lock = shared.lock().unwrap();
    // }
    // let lock = shared.lock().unwrap();

    std::mem::drop(lock);

    // we can use try_lock() to attempt to get a lock without blocking
    if let Ok(_lock) = SHARED.try_lock() {
        println!("Lock obtained");
        return;
    }
    println!("No lock - no block!")
}
