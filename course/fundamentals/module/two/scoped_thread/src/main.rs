use std::thread;

fn main() {
   const WORKERS_TOTAL: usize = 8;
   let to_add: Vec<u32> = (0..5000).collect();

   let chunks = to_add.chunks(WORKERS_TOTAL);

  let sum = thread::scope(|s|{
        let mut thread_handles = Vec::new();

        for c in chunks {
            let thread_handle = s.spawn(move || {
                c.iter().sum::<u32>()
            });
            thread_handles.push(thread_handle);
        }

        thread_handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .sum::<u32>()
   });

   println!("Result: {sum}");
}