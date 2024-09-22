//! Todo List

use clap::Parser;
use todo::connection::connect;

fn main() {
    let connection = &mut connect();
}
