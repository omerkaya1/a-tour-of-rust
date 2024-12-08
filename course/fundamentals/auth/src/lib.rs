pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

pub fn login(username: &str, password: &str) -> bool {
    username == "admin" && password == "password"
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap(); // unwrap is used to handle errors

    // trim() removes the newline character at the end of the input
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_greeting() {
        assert_eq!(greeting("world"), "Hello world!");
    }

    #[test]
    fn test_login() {
        assert!(login("admin", "password"));
        assert!(!login("no-admin", "password"));
        assert!(!login("admin", "no-password"));
    }
}
