table! {
    users (id) {
        id -> Integer,
        name -> Text,
        peer_id -> Text,
        online -> Integer,
        secret -> Nullable<Binary>,
    }
}
