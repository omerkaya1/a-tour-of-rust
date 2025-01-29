use std::{fs::File, io::{BufRead, BufReader}};
use memmap2::MmapOptions;

fn main() {
    let now = std::time::Instant::now();
    let file = File::open("warandpeace.txt").unwrap();
    let mmap = unsafe {
        MmapOptions::new().map(&file).unwrap()
    };
    let buffered_reader = BufReader::new(&mmap[..]);
    println!("line count: {}", buffered_reader.lines().count());
    println!("time: {}", now.elapsed().as_millis());
}
