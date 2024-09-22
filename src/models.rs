//! Structs detailing the tables in our DB.

use crate::schema;
use diesel::pg;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::task_state)]
#[diesel(check_for_backend(pg::Pg))]
pub struct TaskState {
    pub id: i32,
    pub description: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::task)]
#[diesel(check_for_backend(pg::Pg))]
pub struct Task {
    pub id: i32,
    pub goal: String,
    pub task_state_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::sub_task_map)]
#[diesel(check_for_backend(pg::Pg))]
pub struct SubTaskMap {
    pub id: i32,
    pub parent_task_id: i32,
    pub sub_task_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::todo_list)]
#[diesel(check_for_backend(pg::Pg))]
pub struct TodoList {
    pub id: i32,
    pub name: String,
    pub owner: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::todo_list_task_map)]
#[diesel(check_for_backend(pg::Pg))]
pub struct TodoListTaskMap {
    pub id: i32,
    pub todo_list_id: i32,
    pub task_id: i32,
}
