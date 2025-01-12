#![allow(dead_code)] // allows for the dead code warnings suppressed on compillation

use async_recursion::*;

struct Some {}

#[async_recursion]
async fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1).await + fib(n - 2).await,
    }
}

#[tokio::main]
async fn main() {
    println!("some");
    println!("fibonacci(10) = {}", fib(10).await);
}
