//! Schema definition for `todo_list`.

// Definition for todo_list.task_state
diesel::table! {
    task_state (id) {
        id -> Int4,
        description -> Varchar,
    }
}
