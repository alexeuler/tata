table! {
    users (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Nullable<Text>,
        peer_id -> Binary,
        private_key -> Binary,
    }
}
