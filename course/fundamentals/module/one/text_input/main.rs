fn main() {
    let input: String = read_user_input();
    println!("You typed: {}", input);
}

fn read_user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap(); // unwrap is used to handle errors

    // trim() removes the newline character at the end of the input
    input.trim().to_string()
}
