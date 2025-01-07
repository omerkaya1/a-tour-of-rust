use futures::{executor::block_on, future::join_all, join};

fn do_cool() {
    println!("not async")
}

async fn hello() {
    println!("hi!");
    join!(second(), goodbye());

    let val = double(7).await;
    println!("{val}");

    let futures = vec![double(1), double(2), double(3)];
    let result = join_all(futures).await;

    println!("{result:?}");

    do_cool();
}

async fn second() {
    println!("hi again!")
}

async fn goodbye() {
    println!("bye!")
}

async fn double(n: u32) -> u32 {
    n * 2
}

fn main() {
    let future = hello();

    block_on(future)
}
