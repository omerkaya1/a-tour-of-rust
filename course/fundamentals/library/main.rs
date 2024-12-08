use auth::{greeting, login, read_line};

fn main() {
	println!("{}", greeting("cruel world"));

	let mut attempts = 3;
	loop {
		println!("Please enter your username:");
		let username = read_line();

		println!("Please enter your password:");
		let password = read_line();
		if login(&username, &password) {
			println!("Welcome, {}!", username);
			return;
		}

		attempts -= 1;
		if attempts == 0 {
			println!("Too many failed login attempts; exiting.");
			return;
		}
		println!("Login failed, please try again.");
	}
}
