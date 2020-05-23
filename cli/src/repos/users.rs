use super::super::models::*;
use diesel::prelude::*;

pub struct UsersRepo;

impl UsersRepo {
    fn list(conn: &SqliteConnection) -> Vec<User> {
        todo!()
    }
}
