//! Todo List

use clap::Parser;
use todo::{
    args::{Cli, Commands, TodoListCommands},
    connection::connect,
    db_operation::DbOperation,
    todo_list::Create,
};

#[must_use]
pub fn db_operation_factory(args: Cli) -> impl DbOperation {
    match args.command {
        Commands::TodoList(commands) => match commands.command {
            TodoListCommands::Create { todo_list, owner } => {
                println!("Calling create with owner {owner} and todo_list {todo_list}");
                Create { owner, todo_list }
            }
            _ => todo!(),
        },
        _ => todo!(),
    }
}

fn main() {
    let connection = &mut connect();
    let args = Cli::parse();
    let db_operation = db_operation_factory(args);
    db_operation.operate(connection);
}
