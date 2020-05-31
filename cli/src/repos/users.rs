use super::super::models::*;
use crate::schema::users::{self, dsl::*};
use diesel::prelude::*;

pub struct UsersRepo<'a> {
    conn: &'a SqliteConnection,
}

impl<'a> UsersRepo<'a> {
    pub fn new(conn: &'a SqliteConnection) -> UsersRepo<'a> {
        UsersRepo { conn }
    }

    pub fn list(&self) -> QueryResult<Vec<User>> {
        users.order(id.desc()).load::<User>(self.conn)
    }

    pub fn create(&self, user: &NewUser) -> QueryResult<()> {
        diesel::insert_into(users::table)
            .values(user)
            .execute(self.conn)?;
        Ok(())
    }

    pub fn update(&self, user_id: i32, user: &UpdateUser) -> QueryResult<()> {
        diesel::update(users.find(user_id))
            .set(user)
            .execute(self.conn)?;
        Ok(())
    }

    pub fn delete(&self, user_id: i32) -> QueryResult<()> {
        diesel::delete(users.find(user_id)).execute(self.conn)?;
        Ok(())
    }
}
