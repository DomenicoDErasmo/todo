//! Library of modules in the crate.
pub mod args;
pub mod connection;
pub mod db_operation;
pub mod models;
#[allow(clippy::pub_use)]
#[allow(clippy::single_char_lifetime_names)]
#[allow(clippy::missing_trait_methods)]
#[allow(clippy::missing_inline_in_public_items)]
pub mod schema;
pub mod todo_list;
