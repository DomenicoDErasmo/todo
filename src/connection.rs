//! Establishing a connection to the `PostgreSQL` server.

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

/// Connects to the `PostgreSQL` database.
///
/// ### Panics
/// Panics if `DATABASE_URL` is not set in the .env file or if the connection could not be made.
#[inline]
#[must_use]
pub fn connect() -> PgConnection {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .expect("Failed to connect to the server.")
}
