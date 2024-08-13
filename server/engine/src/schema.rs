// @generated automatically by Diesel CLI.

diesel::table! {
    edge (id) {
        id -> Int4,
        from_vertex_id -> Int4,
        #[max_length = 255]
        from_vertex_type -> Varchar,
        to_vertex_id -> Int4,
        #[max_length = 255]
        to_vertex_type -> Varchar,
        #[max_length = 255]
        label -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        created_by -> Varchar,
        #[max_length = 255]
        updated_by -> Varchar,
    }
}

diesel::table! {
    vertex (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[sql_name = "type"]
        #[max_length = 255]
        type_ -> Varchar,
        created_at -> Timestamp,
        #[max_length = 255]
        created_by -> Varchar,
        updated_at -> Timestamp,
        #[max_length = 255]
        updated_by -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    edge,
    vertex,
);
