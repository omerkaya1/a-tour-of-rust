fn main() {
    const WORKERS_TOTAL: usize = 8;
    let to_add: Vec<u32> = (0..5000).collect();

    // NOTE: a longer form is needed if we do not specify the type for the sum func
    // let mut thread_handles: Vec<std::thread::JoinHandle<u32>> = Vec::new(); 
    let mut thread_handles = Vec::new();

    let chunks = to_add.chunks(WORKERS_TOTAL);

    for chunk in chunks {
        let temp_chunk = chunk.to_owned();

        thread_handles.push(std::thread::spawn(move || {
            // temp_chunk.iter().sum() <- this does not allow for the type inference and requires a longer form for type declaration
            temp_chunk.iter().sum::<u32>()
        }));
    }

    let mut sum = 0;
    for h in thread_handles {
        sum += h.join().unwrap();
    }

    println!("Sum is: {}", sum);
}
