async fn hello1() -> u32 {
    println!("hello tokio! 1");
    1
}

async fn hello2() -> u32 {
    println!("hello tokio! 2");
    2
}

async fn ticker() {
    for i in 0..10 {
        println!("tick {i}");
        tokio::task::yield_now().await;
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // let (first, second) = tokio::join!(hello1(), hello2());
    // println!("{first}, {second}");
    
    // tokio::spawn(ticker());
    // hello1().await;

    let _ = tokio::join!(
        tokio::spawn(hello1()),
        tokio::spawn(ticker()),
        tokio::spawn(ticker()),
    );
    println!("finished!")
}
