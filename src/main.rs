//! Todo List

use clap::Parser;
use todo::{
    args::{Cli, Commands, TaskCommands, TodoListCommands},
    connection::connect,
    db_operation::DbOperation,
    models::TaskStatus,
    task, todo_list,
};

/// Creates the DB operation command based on CLI args.
///
/// ### Parameters
/// * `args`: The arguments passed from the CLI.
///
/// ### Returns
/// * The DB operation to execute.
///
/// ### Panics
/// * Panics if an invalid string is passed for updating a task's `TaskStatus`.
#[must_use]
#[allow(clippy::expect_fun_call)]
pub fn db_operation_factory(args: Cli) -> Box<dyn DbOperation> {
    match args.command {
        Commands::TodoList(commands) => match commands.command {
            TodoListCommands::Create { todo_list, owner } => {
                Box::new(todo_list::create::Create { owner, todo_list })
            }
            TodoListCommands::Read { todo_list, owner } => {
                Box::new(todo_list::read::Read { todo_list, owner })
            }
            TodoListCommands::Delete { todo_list, owner } => {
                Box::new(todo_list::delete::Delete { todo_list, owner })
            }
        },
        Commands::Task(commands) => match commands.command {
            TaskCommands::Create {
                todo_list,
                owner,
                goal: task,
            } => Box::new(task::create::Create {
                todo_list,
                owner,
                task,
            }),
            TaskCommands::Update {
                todo_list,
                owner,
                goal: task,
                name,
                status,
            } => {
                let task_status = status.map(|val| {
                    TaskStatus::try_from(val.clone()).expect(
                        format!("Failed to convert {val} to a `TaskStatus`.")
                            .as_str(),
                    )
                });
                Box::new(task::update::Update {
                    todo_list,
                    owner,
                    goal: task,
                    new_goal: name,
                    new_status: task_status,
                })
            }
            TaskCommands::Delete {
                todo_list,
                owner,
                goal,
            } => Box::new(task::delete::Delete {
                todo_list,
                owner,
                goal,
            }),
            TaskCommands::Read {
                todo_list,
                owner,
                goal,
            } => Box::new(task::read::Read {
                todo_list,
                owner,
                goal,
            }),
        },
    }
}

fn main() {
    let connection = &mut connect();
    let args = Cli::parse();
    let db_operation = db_operation_factory(args);
    db_operation.operate(connection);
}
