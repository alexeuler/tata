use super::peer_id::PeerId;
use super::secret::Secret;
use crate::ffi::generate_pair;
use crate::schema::users;
use diesel::Queryable;

#[derive(Debug, Queryable)]
pub struct User {
    id: i32,
    first_name: String,
    last_name: Option<String>,
    peer_id: PeerId,
    secret: Secret,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    first_name: String,
    last_name: Option<String>,
    peer_id: PeerId,
    secret: Secret,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl NewUser {
    pub fn new(first_name: String, last_name: Option<String>) -> Self {
        let (secret, peer_id) = generate_pair();
        NewUser {
            first_name,
            last_name,
            peer_id,
            secret,
        }
    }
}
