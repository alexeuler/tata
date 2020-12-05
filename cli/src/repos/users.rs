use super::super::models::*;
use crate::schema::users::{self, dsl::*};
use diesel::prelude::*;

/// Persistent storage for users
pub trait UsersRepo {
    fn list(&self) -> QueryResult<Vec<User>>;
    fn count(&self) -> QueryResult<i64>;
    fn local_users(&self) -> QueryResult<Vec<User>>;
    fn find(&self, user_id: i32) -> QueryResult<Option<User>>;
    fn create(&self, user: &NewUser) -> QueryResult<()>;
    fn update(&self, user_id: i32, user: &UpdateUser) -> QueryResult<()>;
    fn delete(&self, user_id: i32) -> QueryResult<()>;
}

pub struct UsersRepoImpl<'a> {
    conn: &'a SqliteConnection,
}

impl<'a> UsersRepo for UsersRepoImpl<'a> {
    fn list(&self) -> QueryResult<Vec<User>> {
        users.order(id.desc()).load::<User>(self.conn)
    }

    fn count(&self) -> QueryResult<i64> {
        users.count().get_result(self.conn)
    }

    fn local_users(&self) -> QueryResult<Vec<User>> {
        users
            .filter(secret.is_not_null())
            .order(id.desc())
            .load::<User>(self.conn)
    }

    fn find(&self, user_id: i32) -> QueryResult<Option<User>> {
        users.find(user_id).first(self.conn).optional()
    }

    fn create(&self, user: &NewUser) -> QueryResult<()> {
        diesel::insert_into(users::table)
            .values(user)
            .execute(self.conn)?;
        Ok(())
    }

    fn update(&self, user_id: i32, user: &UpdateUser) -> QueryResult<()> {
        diesel::update(users.find(user_id))
            .set(user)
            .execute(self.conn)?;
        Ok(())
    }

    fn delete(&self, user_id: i32) -> QueryResult<()> {
        diesel::delete(users.find(user_id)).execute(self.conn)?;
        Ok(())
    }
}

impl<'a> UsersRepoImpl<'a> {
    /// Create new instance
    pub fn new(conn: &'a SqliteConnection) -> UsersRepoImpl<'a> {
        Self { conn }
    }
}
