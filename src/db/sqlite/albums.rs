use sqlx::{Pool, Sqlite};
use std::{future::Future, pin::Pin};

use crate::db::{content::Album, Table};

pub struct AlbumTable {
    pub pool: Pool<Sqlite>,
}

impl<Q: ToString + Send + 'static> Table<Q> for AlbumTable {
    type Item = Album;
    type Database = Sqlite;

    fn get(&self, id: Q) -> Pin<Box<dyn Future<Output = Option<Self::Item>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM albums where id = $1;";

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

    fn get_many(&self, id: Q) -> Pin<Box<dyn Future<Output = Vec<Self::Item>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM albums where id = $1;";

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
        let query = "SELECT * FROM albums;";

        Box::pin(async move {
            sqlx::query_as::<Self::Database, Self::Item>(query)
                .fetch_all(&pool)
                .await
                .unwrap()
        })
    }

    fn save(&self, item: Self::Item) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let pool = self.pool.clone();
        let query = "INSERT INTO albums (id, name) VALUES (?, ?);";

        Box::pin(async move {
            sqlx::query::<Self::Database>(query)
                .bind(item.id.to_string())
                .bind(item.name.to_string())
                // .bind(item.cover)
                // TODO!() implemente release date
                .execute(&pool)
                .await
                .unwrap();
        })
    }

    fn save_many(&self, _items: Vec<Self::Item>) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        todo!()
    }
}
