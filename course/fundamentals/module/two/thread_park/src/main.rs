fn parkable(i: u32) {
    loop {
        std::thread::park();
        println!("parkable {i} unparked");
    }
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut threads = Vec::new();
    for i in 0..10 {
        let thread = std::thread::spawn(move || {
            parkable(i);
        });

        threads.push(thread);
    }

    loop {
        println!("thread to unpark: ");
        let input = read_line();
        if input == "q" {
            break;
        }

        if let Ok(num) = input.parse::<usize>() {
            if num < 10 {
                threads[num].thread().unpark();
            }
        }
    }
}
