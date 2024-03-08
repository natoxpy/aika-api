use crate::db::{sqlite::SqliteTables, Tables};

pub const FILES: &str = "/home/toxpy/.aika";

pub struct DB<T: sqlx::Database = sqlx::Sqlite> {
    pub tables: Box<dyn Tables<Database = T>>,
    pub pool: sqlx::Pool<T>,
}

impl<T: sqlx::Database> DB<T> {
    pub fn new(tables: Box<dyn Tables<Database = T>>, pool: sqlx::Pool<T>) -> Self {
        Self { tables, pool }
    }
}

impl DB {
    pub fn new_sqlite(pool: sqlx::Pool<sqlx::Sqlite>) -> Self {
        let tables = SqliteTables::new(pool.clone());
        Self::new(tables, pool)
    }
}
