#![allow(dead_code)] // allows for the dead code warnings suppressed on compillation

use std::{
    pin::Pin,
};
use futures::future::Future;
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

async fn one() {
    println!("one");
}

async fn two() {
    println!("two");
}

async fn call_either(n: u32) -> Pin<Box<dyn Future<Output = ()>>> {
    println!("HERE!");
    match n {
        1 => Box::pin(one()),
        2 => Box::pin(two()),
        _ => panic!("PPAAAAANICCC!")
    }
}

#[tokio::main]
async fn main() {


    println!("fibonacci(10) = {}", fib(10).await);

    let future = async {
        println!("hi!");
    };

    tokio::pin!(future);

    (&mut future).await
}
