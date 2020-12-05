use super::peer_id::PeerId;
use super::secret::Secret;
use crate::core::generate_keypair;
use crate::schema::users;
use diesel::Queryable;

#[derive(Debug, Queryable, Clone, Default)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub peer_id: PeerId,
    pub online: i32,
    pub secret: Option<Secret>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    name: String,
    peer_id: PeerId,
    online: i32,
    secret: Option<Secret>,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    pub name: Option<String>,
    pub online: Option<i32>,
}

impl NewUser {
    /// Creates a local user with secret
    pub fn new(name: String) -> Self {
        let (secret, peer_id) = generate_keypair();
        NewUser {
            name,
            peer_id,
            online: 0,
            secret: Some(secret),
        }
    }

    /// Creates a peer record in the database
    pub fn new_peer(name: String, peer_id: PeerId) -> Self {
        NewUser {
            name,
            peer_id,
            online: 0,
            secret: None,
        }
    }
}
