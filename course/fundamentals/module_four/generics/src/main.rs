#![allow(dead_code, unused)]

use std::fmt::Debug;

// the below notation is similar to:
// fn print_stuff<>(x: T)
// where T: ToString,
// {
// ...
fn print_stuff<T: ToString + Debug>(x: T) {
    println!("{}", x.to_string())
}

fn main() {
    print_stuff("hello!");
    print_stuff(7);
}
