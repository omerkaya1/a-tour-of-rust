use auth::{get_authorised_users_map, get_autorised_users};
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

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            list_users();
        }
        None => {
            println!("Run with --help to see instructions");
        }
    }
}
