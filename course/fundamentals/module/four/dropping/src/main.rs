#![allow(dead_code)]

struct Struct {
    n: i32,
}

impl Struct {
    fn new(n: i32) -> Self {
        println!("constructing {n}");
        Self { n }
    }
}

impl Drop for Struct {
    fn drop(&mut self) {
        println!("dropping {}", self.n);
    }
}

fn move_it(x: Struct) {
    // do cool
}

struct HasDroppable {
    s: Struct,
}

fn main() {
    let x = Struct::new(5);
    {
        let x2 = Struct::new(2);
    }
    move_it(x);
    println!("got back to the main");

    let droppable = HasDroppable{s: Struct::new(4)};

    println!("done");
}
