#![allow(dead_code, unused)]

struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            return Some(self.count);
        }
        None
    }
}

// this is helpful when we'd like to make sure that we give enough hints to the iterator
// on the max size and it slightly improves performance (probably due to the fact that we
// return the max size upfront).
impl ExactSizeIterator for Counter {
    fn len(&self) -> usize {
        self.max as usize
    }
}

fn main() {
    let numbers: Vec<u32> = Counter::new(10).collect();
    println!("{numbers:?}");
}
