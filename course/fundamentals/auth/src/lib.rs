pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginEntities),
    Denied,
}

#[derive(PartialEq, Eq, Debug)]
pub enum LoginEntities {
    Admin,
    User,
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {    
    let username = username.to_lowercase();

    if username != "admin" || username != "some" {
        return None;
    }
 
    if username == "admin" && password == "password" {
        return Some(LoginAction::Granted(LoginEntities::Admin));
    }
    if username == "some" && password == "password" {
        return Some(LoginAction::Granted(LoginEntities::User));
    }
    Some(LoginAction::Denied)
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
        assert_eq!(login("admin", "password"), Some(LoginAction::Granted(LoginEntities::Admin)));
        assert_eq!(login("some", "password"), Some(LoginAction::Granted(LoginEntities::User)));
        assert_eq!(login("no-admin", "password"), Some(LoginAction::Denied));
        assert_eq!(login("admin", "no-password"), Some(LoginAction::Denied));
    }
}
