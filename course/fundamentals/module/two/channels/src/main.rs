use std::sync::mpsc;

enum Cmd {
    SayHello,
    Quit,
}

fn main() {
    let (tx, rx) = mpsc::channel::<Cmd>();

    let handle = std::thread::spawn(move || {
        while let Ok(cmd) = rx.recv() {
            match cmd {
                Cmd::SayHello => println!("Hello!"),
                Cmd::Quit => {
                    println!("quit");
                    break;
                }
            }
        }
    });

    for _ in 0..10 {
        tx.send(Cmd::SayHello).unwrap();
    }
    println!("sending a quit cmd");
    tx.send(Cmd::Quit).unwrap();
    handle.join().unwrap();
}
