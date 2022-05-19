table! {
    implants (implant_id) {
        implant_id -> Uuid,
        public_key -> Text,
        server_private_key -> Text,
        created_at -> Timestamp,
    }
}

table! {
    results (result_id) {
        result_id -> Uuid,
        result_content -> Text,
        created_at -> Timestamp,
        task_id -> Uuid,
    }
}

table! {
    tasks (task_id) {
        task_id -> Uuid,
        task -> Text,
        created_at -> Timestamp,
        status -> Text,
        implant_id -> Uuid,
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

joinable!(results -> tasks (task_id));
joinable!(tasks -> implants (implant_id));

allow_tables_to_appear_in_same_query!(
    implants,
    results,
    tasks,
    users,
);
