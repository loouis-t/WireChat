// @generated automatically by Diesel CLI.

diesel::table! {
    messages (sender_public_key) {
        sender_public_key -> Text,
        receiver_public_key -> Text,
        message -> Text,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    peers (public_key) {
        public_key -> Text,
        name -> Text,
        endpoint -> Text,
        allowed_ip -> Text,
        interface_ip -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    peers,
);
