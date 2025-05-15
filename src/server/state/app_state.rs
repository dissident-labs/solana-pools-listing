use crate::database::Database;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub(crate) db_conn: Arc<Database>,
}

impl AppState {
    pub fn new(db_conn: Arc<Database>) -> AppState {
        Self { db_conn }
    }
}
impl Default for AppState {
    fn default() -> Self {
        Self {
            db_conn: Arc::new(Database::default()),
        }
    }
}
