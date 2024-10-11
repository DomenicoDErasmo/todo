//! Structs detailing the tables in our DB.

use core::fmt::Error;

use crate::schema;
use crate::schema::task;
use crate::schema::todo_list;
use diesel::pg;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::task_state)]
#[diesel(check_for_backend(pg::Pg))]
pub struct TaskState {
    pub id: i32,
    pub description: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::task)]
#[diesel(check_for_backend(pg::Pg))]
pub struct Task {
    pub id: i32,
    pub goal: String,
    pub task_state_id: i32,
    pub todo_list_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name=task)]
pub struct NewTask {
    pub goal: String,
    pub task_state_id: i32,
    pub todo_list_id: i32,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::todo_list)]
#[diesel(check_for_backend(pg::Pg))]
pub struct TodoList {
    pub id: i32,
    pub name: String,
    pub owner: String,
}

#[derive(Insertable)]
#[diesel(table_name=todo_list)]
pub struct NewTodoList {
    pub name: String,
    pub owner: String,
}

#[derive(Clone, Copy)]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Completed,
}

impl From<TaskStatus> for i32 {
    #[inline]
    fn from(value: TaskStatus) -> Self {
        match value {
            TaskStatus::NotStarted => 1,
            TaskStatus::InProgress => 2,
            TaskStatus::Completed => 3,
        }
    }
}

impl TryFrom<i32> for TaskStatus {
    type Error = Error;

    #[inline]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::NotStarted),
            2 => Ok(Self::InProgress),
            3 => Ok(Self::Completed),
            _ => Err(Error),
        }
    }
}

// TODO: how to make for better entry on CLI?
impl From<TaskStatus> for String {
    #[inline]
    fn from(value: TaskStatus) -> Self {
        match value {
            TaskStatus::NotStarted => "Not Started".to_owned(),
            TaskStatus::InProgress => "In Progress".to_owned(),
            TaskStatus::Completed => "Completed".to_owned(),
        }
    }
}

// TODO: how to make for better entry on CLI?
impl TryFrom<String> for TaskStatus {
    type Error = Error;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Not Started" => Ok(Self::NotStarted),
            "In Progress" => Ok(Self::InProgress),
            "Completed" => Ok(Self::Completed),
            _ => Err(Error),
        }
    }
}
