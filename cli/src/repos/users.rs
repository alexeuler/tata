use super::super::models::*;
use crate::schema::users::dsl::*;
use diesel::prelude::*;

pub struct UsersRepo;

impl UsersRepo {
    pub fn list(conn: &SqliteConnection) -> QueryResult<Vec<User>> {
        users.load::<User>(conn)
    }
}
