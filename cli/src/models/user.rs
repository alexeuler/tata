use super::peer_id::PeerId;
use super::secret::Secret;
use crate::core::generate_keypair;
use crate::schema::users;
use diesel::Queryable;

#[derive(Debug, Queryable, Clone, Default)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub peer_id: PeerId,
    pub secret: Secret,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    name: String,
    peer_id: PeerId,
    secret: Secret,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    pub name: Option<String>,
}

impl NewUser {
    pub fn new(name: String) -> Self {
        let (secret, peer_id) = generate_keypair();
        NewUser {
            name,
            peer_id,
            secret,
        }
    }
}
