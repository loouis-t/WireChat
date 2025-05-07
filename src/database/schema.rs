// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Nullable<Integer>,
        sender_public_key -> Text,
        receiver_public_key -> Text,
        message -> Text,
        timestamp -> Nullable<Timestamp>,
    }
}

diesel::table! {
    peers (public_key) {
        public_key -> Nullable<Text>,
        name -> Nullable<Text>,
        endpoint -> Text,
        allowed_ip -> Text,
        interface_ip -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    peers,
);
