table! {
    implants (implant_id) {
        implant_id -> Uuid,
        public_key -> Text,
        server_private_key -> Text,
        created_at -> Timestamp,
        external_ip_address -> Nullable<Text>,
        internal_ip_address -> Nullable<Text>,
        os_type -> Nullable<Text>,
        machine_user -> Nullable<Text>,
        machine_name -> Nullable<Text>,
        process_name -> Nullable<Text>,
        pid -> Nullable<Int4>,
        architecture -> Nullable<Text>,
    }
}

table! {
    tasks (task_id) {
        task_id -> Uuid,
        task -> Text,
        task_created_at -> Timestamp,
        task_status -> Text,
        result_content -> Nullable<Text>,
        result_nonce -> Nullable<Text>,
        result_created_at -> Nullable<Timestamp>,
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

joinable!(tasks -> implants (implant_id));

allow_tables_to_appear_in_same_query!(
    implants,
    tasks,
    users,
);
