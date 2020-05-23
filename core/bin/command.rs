use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Working with users
    User(UserCommand),
}

#[derive(Debug, StructOpt)]
pub enum UserCommand {
    /// List all users
    List,
    /// Create new user
    Create {
        /// First name of a user
        #[structopt(short, long)]
        first_name: String,
        /// Last name of a user
        #[structopt(short, long)]
        last_name: Option<String>,
    },
    /// Update user details
    Update {
        /// Id of a user to update
        #[structopt(short, long)]
        id: i32,
        /// First name of a user
        #[structopt(short, long)]
        first_name: Option<String>,
        /// Last name of a user
        #[structopt(short, long)]
        last_name: Option<String>,
    },
    /// Delete user
    Delete {
        /// Id of a user to delete
        #[structopt(short, long)]
        id: i32,
    },
}
