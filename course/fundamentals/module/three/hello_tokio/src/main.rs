// use tokio::runtime;

async fn hello() {
    println!("hello tokio!")
}

#[tokio::main]
async fn main() {
    hello().await
}

// fn main() {
//     let rt = runtime::Builder::new_multi_thread()
//         .enable_all()
//         .worker_threads(4)
//         .build()
//         .unwrap();

//     rt.block_on(hello());
// }