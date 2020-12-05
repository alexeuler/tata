table! {
    users (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Nullable<Text>,
        peer_id -> Text,
        secret -> Binary,
    }
}
