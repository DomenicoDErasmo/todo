//! Arg parsing for the crate

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "todo_list")]
#[command(about = "A todo list CLI tool.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    TodoList(TodoListArguments),
    Task(TaskArguments),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct TodoListArguments {
    #[command(subcommand)]
    pub command: TodoListCommands,
}

#[derive(Debug, Subcommand)]
pub enum TodoListCommands {
    Create { todo_list: String, owner: String },
    Delete { todo_list: String },
    View { todo_list: String },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct TaskArguments {
    #[command(subcommand)]
    pub command: TaskCommands,
}

#[derive(Debug, Subcommand)]
pub enum TaskCommands {
    Add {
        todo_list: String,
        task: String,
    },
    // TODO: how to make status be an enum of TaskStatus that gets used by Diesel to populate table?
    Update {
        todo_list: String,
        task: String,
        name: Option<String>,
        status: Option<String>,
    },
    Remove {
        todo_list: String,
        task: String,
    },
    View {
        todo_list: String,
        task: String,
    },
}
