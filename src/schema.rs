table! {
    implants (implant_id) {
        implant_id -> Uuid,
        public_key -> Text,
        created_at -> Timestamp,
    }
}

table! {
    users (user_id) {
        user_id -> Uuid,
        username -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    implants,
    users,
);
