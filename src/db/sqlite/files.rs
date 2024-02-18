use sqlx::{ColumnIndex, Pool, Row, Sqlite};
use std::{future::Future, pin::Pin};

use crate::db::{content::File, Table};

pub struct ArtistTable {
    pub pool: Pool<Sqlite>,
}

impl<'r, R> Table<'r, R> for ArtistTable
where
    R: Row,
    &'r str: ColumnIndex<R>,
    String: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
    i64: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
{
    type Item = File;
    type Database = Sqlite;

    fn get<Q: ToString + Send + 'static>(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Option<Self::Item>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM files where id = $1;";

        Box::pin(async move {
            if let Ok(item) = sqlx::query_as::<Self::Database, Self::Item>(query)
                .bind(id.to_string())
                .fetch_one(&pool)
                .await
            {
                Some(item)
            } else {
                None
            }
        })
    }

    fn get_many<Q: ToString + Send + 'static>(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Vec<Self::Item>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM files where id = $1;";

        Box::pin(async move {
            sqlx::query_as::<Self::Database, Self::Item>(query)
                .bind(id.to_string())
                .fetch_all(&pool)
                .await
                .unwrap()
        })
    }

    fn get_all(&self) -> Pin<Box<dyn Future<Output = Vec<Self::Item>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM files;";

        Box::pin(async move {
            sqlx::query_as::<Self::Database, Self::Item>(query)
                .fetch_all(&pool)
                .await
                .unwrap()
        })
    }

    fn save(&self, item: Self::Item) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let pool = self.pool.clone();
        let query = "INSERT INTO files (id, name, location, size, mime) VALUES (?, ?, ?, ?, ?);";

        Box::pin(async move {
            sqlx::query::<Self::Database>(query)
                .bind(item.id.to_string())
                .bind(item.name)
                .bind(item.location)
                .bind(item.size as i64)
                .bind(item.mime)
                .execute(&pool)
                .await
                .unwrap();
        })
    }

    fn save_many(&self, _items: Vec<Self::Item>) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        todo!()
    }
}
