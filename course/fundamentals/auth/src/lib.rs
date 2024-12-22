use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

pub fn hash_pwd(pwd: &str) -> String {
    use sha2::Digest;

    let mut hasher = sha2::Sha256::new();
    hasher.update(pwd);

    format!("{:X}", hasher.finalize())
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum LoginAction {
    Granted(LoginEntities),
    Denied,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginEntities {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginEntities,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginEntities) -> User {
        User {
            username: username.to_lowercase(),
            password: hash_pwd(password),
            role,
        }
    }
}

// [User; 2] <- array
// Vec<User> <- vector
pub fn get_autorised_users() -> Vec<User> {
    vec![
        User::new("admin", "password", LoginEntities::Admin),
        User::new("some", "password", LoginEntities::User),
    ]
}

pub fn get_authorised_users_map() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert(
        "admin".to_string(),
        User::new("admin", "password", LoginEntities::Admin),
    );
    users.insert(
        "some".to_string(),
        User::new("some", "password", LoginEntities::User),
    );
    users
}

pub fn save_users(users: HashMap<String, User>) {
    let users_path = Path::new("users.json");
    let users_json = serde_json::to_string(&users).unwrap();
    std::fs::write(users_path, users_json).unwrap();
}

pub fn get_default_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");

    if users_path.exists() {
        let users_serialized = std::fs::read_to_string(users_path).unwrap();
        let users = serde_json::from_str(&users_serialized).unwrap();
        return users;
    }
    let users = get_authorised_users_map();
    let users_json = serde_json::to_string(&users).unwrap();
    std::fs::write(users_path, users_json).unwrap();
    users
}

fn get_admin_users() -> Vec<String> {
    get_autorised_users()
        .into_iter()
        .filter(|user| user.role == LoginEntities::Admin)
        .map(|user| user.username)
        .collect()
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let password = hash_pwd(password);

    // using vectorised solution
    // let users = get_autorised_users();
    // if let Some(user) = users.iter().find(|user| user.username == username) {
    //     if user.password == password {
    //         return Some(LoginAction::Granted(user.role.clone())); // cloning capability is defined within the derive directive.
    //     }
    //     return Some(LoginAction::Denied);
    // }

    // using a map
    // let users = get_authorised_users_map();
    // serialization
    let users = get_default_users();
    if let Some(user) = users.get(&username) {
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
        assert_eq!(
            login("admin", "password"),
            Some(LoginAction::Granted(LoginEntities::Admin))
        );
        assert_eq!(
            login("some", "password"),
            Some(LoginAction::Granted(LoginEntities::User))
        );
        assert_eq!(login("no-admin", "password"), Some(LoginAction::Denied));
        assert_eq!(login("admin", "no-password"), Some(LoginAction::Denied));
    }

    #[test]
    fn test_get_admin_users() {
        assert_eq!(get_admin_users(), vec!["admin"])
    }
}
