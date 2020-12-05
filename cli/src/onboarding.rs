//! Scripts used on the initial run of the app

use crate::prelude::*;
use crate::repos::UsersRepo;

pub async fn onboard_if_necessary(users_repo: &dyn UsersRepo) -> Result<()> {
    println!("{}", is_initial_run(users_repo)?);
    Ok(())
}

fn is_initial_run(users_repo: &dyn UsersRepo) -> Result<bool> {
    Ok(users_repo.count()? == 0)
}
