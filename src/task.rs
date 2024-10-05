//! Module for defining DB operations on a task.

use crate::db_operation::DbOperation;

pub struct Create {
    pub todo_list: String,
    pub task: String,
}

impl DbOperation for Create {
    #[inline]
    fn operate(&self, connection: &mut diesel::PgConnection) {
        
    }
}
