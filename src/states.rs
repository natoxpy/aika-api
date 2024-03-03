use crate::db::{sqlite::SqliteTables, Tables};

pub const FILES: &'static str = "/home/toxpy/.aika";

pub struct DB<T: sqlx::Database = sqlx::Sqlite> {
    pub tables: Box<dyn Tables<Database = T>>,
}

impl<T: sqlx::Database> DB<T> {
    pub fn new(tables: Box<dyn Tables<Database = T>>) -> Self {
        Self { tables }
    }
}

impl DB {
    pub fn new_sqlite(pool: sqlx::Pool<sqlx::Sqlite>) -> Self {
        let tables = SqliteTables::new(pool);
        Self::new(tables)
    }
}
