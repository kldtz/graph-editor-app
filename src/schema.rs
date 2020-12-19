table! {
    edge (id) {
        id -> Int4,
        edge_label -> Text,
        source_id -> Int4,
        target_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    graph (id) {
        id -> Int4,
        graph_label -> Text,
        translate_x -> Float8,
        translate_y -> Float8,
        scale -> Float8,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    node (id) {
        id -> Int4,
        node_label -> Text,
        graph_id -> Int4,
        x_coord -> Float8,
        y_coord -> Float8,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

joinable!(node -> graph (graph_id));

allow_tables_to_appear_in_same_query!(
    edge,
    graph,
    node,
);
