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

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum LoginEntities {
    Admin,
    User,
}

pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginEntities,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginEntities) -> User {
        User { 
            username: username.to_lowercase(), 
            password: password.to_lowercase(), 
            role 
        }
    }
}

// [User; 2] <- array
// Vec<User> <- vector
pub fn get_autorised_users() -> Vec<User> {
    vec![
        User::new("admin", "password", LoginEntities::Admin),
        User::new("some", "password", LoginEntities::User)
    ]
}

fn get_admin_users() -> Vec<String> {
    get_autorised_users().
        into_iter().
        filter(|user| user.role == LoginEntities::Admin).
        map(|user| user.username).
        collect()
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {    
    let username = username.to_lowercase();
    let users = get_autorised_users();

    if let Some(user) = users.iter().find(|user| user.username == username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone())); // cloning capability is defined within the derive directive.
        }
        return Some(LoginAction::Denied);
    }
    None
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


    #[test]
    fn test_get_admin_users() {
        assert_eq!(get_admin_users(), vec!["admin"])
    }

}
