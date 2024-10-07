//! Module for reading a task in some todo list.

use crate::db_operation::DbOperation;
use crate::models::{Task, TodoList};
use crate::schema::task::dsl::task;
use crate::schema::task::{goal, todo_list_id};
use crate::schema::todo_list::dsl::todo_list;
use crate::schema::todo_list::{name, owner};
use diesel::prelude::*;

pub struct Read {
    pub todo_list: String,
    pub owner: String,
    pub goal: String,
}

// TODO: techinically we should be returning a value here for downstream code to work with
// Adding a return type will mess with the trait - how to proceed?
impl DbOperation for Read {
    #[inline]
    fn operate(&self, connection: &mut diesel::PgConnection) {
        let queried_todo_list_id = todo_list
            .filter(name.eq(self.todo_list.clone()))
            .filter(owner.eq(self.owner.clone()))
            .limit(1)
            .select(TodoList::as_select())
            .load(connection)
            .expect("Error loading task.")
            .first()
            .expect("Failed to get one todo list from the query.")
            .id;

        let task_query = task
            .filter(todo_list_id.eq(queried_todo_list_id))
            .filter(goal.eq(self.goal.clone()))
            .limit(1)
            .select(Task::as_select())
            .load(connection)
            .expect("Error loading connection.");

        let queried_task = task_query
            .first()
            .expect("Failed to get one task from the query");

        println!("{queried_task:#?}");
    }
}