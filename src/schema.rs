// @generated automatically by Diesel CLI.

diesel::table! {
    sub_task_map (id) {
        id -> Int4,
        parent_task_id -> Int4,
        sub_task_id -> Int4,
    }
}

diesel::table! {
    task (id) {
        id -> Int4,
        goal -> Varchar,
        task_state_id -> Int4,
        todo_list_id -> Int4,
    }
}

diesel::table! {
    task_state (id) {
        id -> Int4,
        description -> Varchar,
    }
}

diesel::table! {
    todo_list (id) {
        id -> Int4,
        name -> Varchar,
        owner -> Varchar,
    }
}

diesel::joinable!(task -> task_state (task_state_id));

diesel::allow_tables_to_appear_in_same_query!(
    sub_task_map,
    task,
    task_state,
    todo_list,
);
