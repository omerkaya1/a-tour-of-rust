use serde::Deserialize;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
enum UserError {
    #[error("no users found")]
    NoUsers,
    #[error("too many users found")]
    TooManyUsers,
}

fn read_file() -> Result<String, std::io::Error> {
    let f = Path::new("file.txt");
    std::fs::read_to_string(f)
}

fn uppercase_file_data() -> Result<String, std::io::Error> {
    let data = read_file()?;
    Ok(data.to_uppercase())
}

#[derive(Deserialize)]
struct User {
    user: String,
}

// analogous to anyhow::Result
type GenericType<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn load_users() -> Result<Vec<User>, UserError> {
    let p = Path::new("users.txt");
    let raw = std::fs::read_to_string(p).map_err(|_| UserError::NoUsers)?;
    let users: Vec<User> = serde_json::from_str(&raw).map_err(|_| UserError::NoUsers)?;

    // anyhow::bail!("can't proceed"); // yields an error
    Ok(users)
}

fn main() {
    if let Ok(content) = uppercase_file_data() {
        println!("contents: {content}");
    }

    let f = Path::new("some.txt");

    let result = std::fs::read_to_string(f);
    match result {
        Ok(result) => println!("{result}"),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => println!("the file was not found"),
            _ => println!("unknown error: {e:#?}"),
        },
    }
}
