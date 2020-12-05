//! Scripts used on the initial run of the app

use crate::repos::UsersRepo;
use crate::{models::NewUser, prelude::*};
use async_std::io;

pub async fn onboard_if_necessary(users_repo: &dyn UsersRepo) -> Result<()> {
    if !is_initial_run(users_repo)? {
        return Ok(());
    };
    println!("It appears there's no active users. We need to create one.");
    print!("Your name: ");
    flush();
    let mut name = String::new();
    io::stdin().read_line(&mut name).await?;
    let user = NewUser::new(name.trim().to_string());
    users_repo.create(&user)?;
    println!("User succesfully created.");
    Ok(())
}

fn is_initial_run(users_repo: &dyn UsersRepo) -> Result<bool> {
    Ok(users_repo.local_users()?.len() == 0)
}

fn flush() {
    let _ = <std::io::Stdout as std::io::Write>::flush(&mut std::io::stdout());
}
