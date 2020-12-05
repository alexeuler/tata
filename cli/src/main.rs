use db::{establish_connection, run_migrations};
use onboarding::onboard_if_necessary;
use prelude::*;
use repos::{UsersRepo, UsersRepoImpl};

#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

mod command_line;
mod core;
mod db;
mod error;
mod models;
mod onboarding;
mod prelude;
mod repos;
mod schema;

#[async_std::main]
async fn main() -> Result<()> {
    let conn = establish_connection()?;
    run_migrations(&conn)?;
    let users_repo = UsersRepoImpl::new(&conn);
    onboard_if_necessary(&users_repo).await?;
    let current_user = users_repo
        .local_users()?
        .pop()
        .ok_or("Unexpected missing local user")?;
    println!("Current user: {:?}", current_user);
    command_line::start_command_line().await;
    Ok(())
}
