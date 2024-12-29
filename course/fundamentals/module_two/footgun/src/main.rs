static mut STATE: i32 = 0;

fn main() {
    let mut handles = Vec::new();

    for _ in 0..1000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..1100 {
                unsafe {
                    STATE += 1;
                }
            }
        });

        handles.push(handle);
    }

    handles.into_iter().for_each(|h| {
        h.join().unwrap()
    });
    unsafe {
        println!("{STATE}");
    }
}
