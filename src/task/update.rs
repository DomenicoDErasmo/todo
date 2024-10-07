//! Module for updating a task.

use crate::models::{Task, TodoList};
use crate::schema::task::dsl::task;
use crate::schema::task::{goal, task_state_id, todo_list_id};
use crate::schema::todo_list::dsl::todo_list;
use crate::schema::todo_list::{name, owner};
use crate::{db_operation::DbOperation, models::TaskStatus};
use diesel::prelude::*;

pub struct Update {
    pub todo_list: String,
    pub owner: String,
    pub task: String,
    pub goal: Option<String>,
    pub status: Option<TaskStatus>,
}

impl DbOperation for Update {
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

        let binding = task
            .filter(todo_list_id.eq(queried_todo_list_id))
            .filter(goal.eq(self.task.clone()))
            .limit(1)
            .select(Task::as_select())
            .load(connection)
            .expect("Error loading connection.");

        let queried_task = binding
            .first()
            .expect("Failed to get one task from the query");

        let new_goal = self
            .goal
            .clone()
            .unwrap_or_else(|| queried_task.goal.clone());
        let new_status_id = self
            .status
            .as_ref()
            .map_or(queried_task.task_state_id, |status| (*status).into());

        let _updated_goal = diesel::update(task.find(queried_task.id))
            .set(goal.eq(new_goal))
            .returning(Task::as_returning())
            .get_result(connection)
            .expect("Failed to update tasks.");
        
        let _updated_task_state_id = diesel::update(task.find(queried_task.id))
            .set(task_state_id.eq(new_status_id))
            .returning(Task::as_returning())
            .get_result(connection)
            .expect("Failed to update tasks.");
    }
}
