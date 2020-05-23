use diesel::Queryable;

#[derive(Queryable)]
pub struct User {
    id: i32,
    first_name: String,
    last_name: Option<String>,
    peer_id: Vec<u8>,
    private_key: Vec<u8>,
}
