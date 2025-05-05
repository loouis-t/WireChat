// @generated automatically by Diesel CLI.

diesel::table! {
    peers (id) {
        id -> Nullable<Integer>,
        public_key -> Text,
        name -> Nullable<Text>,
        endpoint -> Text,
        allowed_ip -> Text,
    }
}
