//! Module for viewing a todo list.

use crate::db_operation::DbOperation;
use crate::models::TodoList;
use crate::schema::todo_list::dsl::todo_list;
use crate::schema::todo_list::{name, owner};
use diesel::prelude::*;

use crate::models::Task;
use crate::schema::task::dsl::task;
use crate::schema::task::todo_list_id;

pub struct Read {
    pub todo_list: String,
    pub owner: String,
}

impl DbOperation for Read {
    #[inline]
    fn operate(&self, connection: &mut diesel::PgConnection) {
        let todo_list_query = todo_list
            .filter(name.eq(self.todo_list.clone()))
            .filter(owner.eq(self.owner.clone()))
            .limit(1)
            .select(TodoList::as_select())
            .load(connection)
            .expect("Error loading task.");
        let queried_todo_list = todo_list_query
            .first()
            .expect("Failed to get one todo list from the query.");

        let task_query = task
            .filter(todo_list_id.eq(queried_todo_list.id))
            .select(Task::as_select())
            .load(connection)
            .expect("Error loading connection.");

        println!("{queried_todo_list:#?}");

        println!("Tasks:");

        for query in task_query {
            println!("{query:#?}");
        }
    }
}
