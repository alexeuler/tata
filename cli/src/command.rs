#[derive(Debug)]
pub enum Command {
    /// List all users
    ListUsers,
    /// Create user
    CreateUser,
    /// Switch user,
    SwitchUser,
    /// Help
    Help,
}

impl std::str::FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split_whitespace();
        let line1 = lines.next().expect("Infallible; qed");
        let res = match line1 {
            "list_users" => Command::ListUsers,
            "create_user" => Command::CreateUser,
            "switch_user" => Command::SwitchUser,
            _ => Command::Help,
        };
        Ok(res)
    }
}

impl Command {
    pub fn help() -> String {
        r#"
COMMANDS:
    list_user           list all users
    create_user         create new user
    switch_user         switch a user
    help                display help
        "#
        .to_string()
    }
}
