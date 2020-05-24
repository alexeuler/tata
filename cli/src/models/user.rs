use crate::schema::users;
use diesel::Queryable;

#[derive(Debug, Queryable)]
pub struct User {
    id: i32,
    first_name: String,
    last_name: Option<String>,
    peer_id: Vec<u8>,
    private_key: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    first_name: String,
    last_name: Option<String>,
    peer_id: Vec<u8>,
    private_key: Vec<u8>,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    first_name: Option<String>,
    last_name: Option<String>,
}
