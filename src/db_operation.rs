//! Database operation trait.

use diesel::PgConnection;

/// Defines some operation against the database.
pub trait DbOperation {
    /// Conducts the struct's defined operation against the database.
    ///
    /// ### Parameters:
    /// * `connection`: A connection to the database.
    fn operate(&self, connection: &mut PgConnection);
}
