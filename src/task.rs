//! Module for defining DB operations on a task.

use diesel::ExpressionMethods;
use diesel::{
    query_dsl::methods::{FilterDsl, LimitDsl, SelectDsl},
    RunQueryDsl, SelectableHelper,
};

use crate::models::{NewTask, Task, TaskStatus};
use crate::schema::task as task_schema;
use crate::{
    db_operation::DbOperation,
    models::TodoList,
    schema::task::dsl::{goal, task, todo_list_id},
    schema::todo_list::dsl::{name, owner, todo_list},
};

pub struct Create {
    pub todo_list: String,
    pub owner: String,
    pub task: String,
}

impl DbOperation for Create {
    #[inline]
    #[allow(clippy::expect_fun_call)]
    fn operate(&self, connection: &mut diesel::PgConnection) {
        let queried_todo_list_id = todo_list
            .filter(name.eq(self.todo_list.clone()))
            .filter(owner.eq(self.owner.clone()))
            .limit(1)
            .select(TodoList::as_select())
            .load(connection)
            .expect("Error loading todo lists.")
            .first()
            .expect(
                format!(
                    "No todo list IDs found with name {} and owner {}.",
                    self.todo_list.clone(),
                    self.owner.clone()
                )
                .as_str(),
            )
            .id;

        // TODO: how to define an enum of values to link to task_state_id?
        let new_task = NewTask {
            goal: self.task.clone(),
            task_state_id: 1,
            todo_list_id: queried_todo_list_id,
        };

        diesel::insert_into(task_schema::table)
            .values(&new_task)
            .returning(Task::as_returning())
            .get_result(connection)
            .expect("Error inserting task.");
    }
}

pub struct Update {
    pub todo_list: String,
    pub owner: String,
    pub task: String,
    pub name: Option<String>,
    pub status: Option<TaskStatus>,
}

impl DbOperation for Update {
    #[inline]
    #[allow(clippy::expect_fun_call)]
    fn operate(&self, connection: &mut diesel::PgConnection) {
        let queried_todo_list_id = todo_list
            .filter(name.eq(self.todo_list.clone()))
            .filter(owner.eq(self.owner.clone()))
            .limit(1)
            .select(TodoList::as_select())
            .load(connection)
            .expect("Error loading todo lists.")
            .first()
            .expect(
                format!(
                    "No todo list IDs found with name {} and owner {}.",
                    self.todo_list.clone(),
                    self.owner.clone()
                )
                .as_str(),
            )
            .id;

        let _queried_task_id = task
            .filter(goal.eq(self.task.clone()))
            .filter(todo_list_id.eq(queried_todo_list_id))
            .limit(1)
            .select(Task::as_select())
            .load(connection)
            .expect("Error loading tasks.")
            .first()
            .expect(
                format!(
                    "No tasks found with name {} in todo list {} with owner {}.",
                    self.task.clone(),
                    self.todo_list.clone(),
                    self.owner.clone()
                )
                .as_str(),
            );

        // TODO: write update statement for task
    }
}
