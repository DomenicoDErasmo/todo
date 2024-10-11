//! Module for creating a todo list.

use diesel::{RunQueryDsl, SelectableHelper};

use crate::db_operation::DbOperation;
use crate::models::{NewTodoList, TodoList};
use crate::schema::todo_list;

pub struct Create {
    pub owner: String,
    pub todo_list: String,
}

impl DbOperation for Create {
    #[inline]
    fn operate(&self, connection: &mut diesel::PgConnection) {
        let todo_list = NewTodoList {
            owner: self.owner.clone(),
            name: self.todo_list.clone(),
        };

        diesel::insert_into(todo_list::table)
            .values(&todo_list)
            .returning(TodoList::as_returning())
            .get_result(connection)
            .expect("Error saving todo list.");
    }
}
