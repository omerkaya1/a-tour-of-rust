use auth::{get_authorised_users_map, get_default_users, save_users, LoginEntities, User};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Lists all user.
    List,
    /// Add users
    Add {
        /// User login
        username: String,
        /// User password (plaintext)
        password: String,
        /// Admin privileges on the user (optional)
        #[arg(long)]
        admin: Option<bool>
    },
    /// Delete users
    Delete {
        /// user login for deletion
        username: String
    },
    /// Change user password
    ChangePWD {
        /// username of the user whose pwd will be changed
        username: String,
        /// the new password
        password: String
    }
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "Role");
    println!("{:-<40}", "");

    let users = get_authorised_users_map();
    users.iter().
        for_each(|(_, user)|{
            println!("{:<20}{:<20?}", user.username, user.role);
        });
}

fn add_user(username: String, password: String, admin: bool) {
    let mut users = get_default_users();
    let role = if admin {
        LoginEntities::Admin
    } else {
        LoginEntities::User
    };

    let user = User::new(&username, &password, role);
    users.insert(username, user);
    save_users(users);
}

fn delete_user(username: String) {
    let mut users = get_default_users();
    if users.contains_key(&username) {
        users.remove(&username);
        save_users(users);
        return;
    }
    println!("{username} does not exist");
}

fn change_pwd(username: String, password: String) {
    let mut users = get_default_users();
    if let Some(user) = users.get_mut(&username) {
        user.password = auth::hash_pwd(&password);
        save_users(users);
        return;
    }
    println!("{username} does not exist");
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            list_users();
        }
        Some(Commands::Add { username, password, admin }) => {
            add_user(username, password, admin.unwrap());
        }
        Some(Commands::Delete { username }) => {
            delete_user(username);
        }
        Some(Commands::ChangePWD { username, password }) => {
            change_pwd(username, password);
        }
        None => {
            println!("Run with --help to see instructions");
        }
    }
}
