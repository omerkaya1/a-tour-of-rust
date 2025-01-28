use auth::{greeting, login, read_line, LoginAction};

fn main() {
	println!("{}", greeting("cruel world"));

	let mut attempts = 3;
	loop {
		println!("Please enter your username:");
		let username = read_line();

		println!("Please enter your password:");
		let password = read_line();

		match login(&username, &password) {
			Some(LoginAction::Granted(role)) => {
				match role {
					auth::LoginEntities::Admin => println!("Admin"),
					auth::LoginEntities::User => println!("User")
				}
				break;
			}
			Some(LoginAction::Denied) => {

			}
			None => {
				println!("None matched!")
			}
		}

		attempts -= 1;
		if attempts == 0 {
			println!("Too many failed login attempts; exiting.");
			return;
		}
		println!("Login failed, please try again.");
	}
}
