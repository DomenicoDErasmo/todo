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
                Box::new(todo_list::Create { owner, todo_list })
            }
            _ => todo!(),
        },
        Commands::Task(commands) => match commands.command {
            TaskCommands::Create {
                todo_list,
                owner,
                task,
            } => Box::new(task::Create {
                todo_list,
                owner,
                task,
            }),
            TaskCommands::Update {
                todo_list,
                owner,
                task,
                name,
                status,
            } => {
                let task_status = status.map(|val| {
                    TaskStatus::try_from(val.clone()).expect(
                        format!("Failed to convert {val} to a `TaskStatus`.")
                            .as_str(),
                    )
                });
                Box::new(task::Update {
                    todo_list,
                    owner,
                    task,
                    name,
                    status: task_status,
                })
            }
            _ => todo!(),
        },
    }
}

fn main() {
    let connection = &mut connect();
    let args = Cli::parse();
    let db_operation = db_operation_factory(args);
    db_operation.operate(connection);
}
