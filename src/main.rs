//! Todo List

use clap::Parser;
use todo::{
    args::{Cli, Commands, TaskCommands, TodoListCommands},
    connection::connect,
    db_operation::DbOperation,
    task, todo_list,
};

#[must_use]
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
