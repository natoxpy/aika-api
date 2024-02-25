use sqlx::{Pool, Sqlite};
use std::{future::Future, pin::Pin};

use crate::db::{content::Artist, Table};

pub struct ArtistTable {
    pub pool: Pool<Sqlite>,
}

impl<Q: ToString + Send + 'static> Table<Q> for ArtistTable {
    type Item = Artist;
    type Database = Sqlite;

    fn get(&self, id: Q) -> Pin<Box<dyn Future<Output = Option<Self::Item>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM artists where id = $1;";

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

    fn get_many(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Self::Item>, crate::db::Error>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM artists where id = $1;";

        Box::pin(async move {
            sqlx::query_as::<Self::Database, Self::Item>(query)
                .bind(id.to_string())
                .fetch_all(&pool)
                .await
                .map_err(|err| crate::db::Error::Sqlx(err))
        })
    }

    fn get_all(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Self::Item>, crate::db::Error>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM artists;";

        Box::pin(async move {
            sqlx::query_as::<Self::Database, Self::Item>(query)
                .fetch_all(&pool)
                .await
                .map_err(|err| crate::db::Error::Sqlx(err))
        })
    }

    fn save(
        &self,
        item: Self::Item,
    ) -> Pin<Box<dyn Future<Output = Result<(), crate::db::Error>> + Send>> {
        let pool = self.pool.clone();
        let query = "INSERT INTO artists (id, name) VALUES (?, ?);";

        Box::pin(async move {
            sqlx::query::<Self::Database>(query)
                .bind(item.id.to_string())
                .bind(item.name.to_string())
                .execute(&pool)
                .await
                .map_err(|err| crate::db::Error::Sqlx(err))?;
            Ok(())
        })
    }

    fn save_many(
        &self,
        _items: Vec<Self::Item>,
    ) -> Pin<Box<dyn Future<Output = Result<(), crate::db::Error>> + Send>> {
        todo!()
    }
}
