#[derive(Debug)]
pub enum Command {
    /// List all users
    ListUsers,
    /// Create user
    CreateUser,
    /// Update user
    UpdateUser,
    /// Delete user
    DeleteUser,
    /// Switch user,
    SwitchUser,
    /// Display current user
    CurrentUser,
    /// Help
    Help,
}

impl std::str::FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split_whitespace();
        let res = if let Some(line1) = lines.next() {
            match line1 {
                "list_users" => Command::ListUsers,
                "create_user" => Command::CreateUser,
                "update_user" => Command::UpdateUser,
                "delete_user" => Command::DeleteUser,
                "switch_user" => Command::SwitchUser,
                "current_user" => Command::CurrentUser,
                _ => Command::Help,
            }
        } else {
            Command::Help
        };
        Ok(res)
    }
}

impl Command {
    pub fn help() -> String {
        r#"
COMMANDS:
    list_users          list all users
    create_user         create new user
    update_user         update a user
    delete_user         delete a user
    switch_user         switch a user
    current_user        display current user
    help                display help
        "#
        .to_string()
    }
}
