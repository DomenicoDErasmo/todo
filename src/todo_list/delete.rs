//! Module for deleting a todo list.

use crate::db_operation::DbOperation;
use crate::models::TodoList;
use crate::schema::todo_list::dsl::todo_list;
use crate::schema::todo_list::{name, owner};
use diesel::prelude::*;

use crate::schema::task::dsl::task;
use crate::schema::task::todo_list_id;

pub struct Delete {
    pub todo_list: String,
    pub owner: String,
}

impl DbOperation for Delete {
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

        let num_tasks_deleted =
            diesel::delete(task.filter(todo_list_id.eq(queried_todo_list.id)))
                .execute(connection)
                .expect("Error deleting tasks.");

        println!(
            "Deleted {num_tasks_deleted} tasks from {} with owner {}.",
            queried_todo_list.name, queried_todo_list.owner
        );

        let _: usize = diesel::delete(todo_list.find(queried_todo_list.id))
            .execute(connection)
            .expect("Failed to delete the todo list.");

        println!(
            "Deleted the todo list {} with owner {}.",
            queried_todo_list.name, queried_todo_list.owner
        );
    }
}
