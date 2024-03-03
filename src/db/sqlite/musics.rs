use crate::db::{content::Music, Table};
use sqlx::{Pool, Sqlite};
use std::{future::Future, pin::Pin};

pub struct MusicTable {
    pub pool: Pool<Sqlite>,
}

impl<Q: ToString + Send + 'static> Table<Q> for MusicTable {
    type Item = Music;
    type Database = Sqlite;

    fn get(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Item, crate::db::Error>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM musics where id = $1;";

        Box::pin(async move {
            sqlx::query_as::<Self::Database, Self::Item>(query)
                .bind(id.to_string())
                .fetch_one(&pool)
                .await
                .map_err(crate::db::Error::Sqlx)
        })
    }

    fn get_many(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Self::Item>, crate::db::Error>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM musics where id = $1;";

        Box::pin(async move {
            sqlx::query_as::<Self::Database, Self::Item>(query)
                .bind(id.to_string())
                .fetch_all(&pool)
                .await
                .map_err(crate::db::Error::Sqlx)
        })
    }

    fn get_all(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Self::Item>, crate::db::Error>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM musics;";

        Box::pin(async move {
            sqlx::query_as::<Self::Database, Self::Item>(query)
                .fetch_all(&pool)
                .await
                .map_err(crate::db::Error::Sqlx)
        })
    }

    fn save(
        &self,
        music: Self::Item,
    ) -> Pin<Box<dyn Future<Output = Result<(), crate::db::Error>> + Send>> {
        let pool = self.pool.clone();
        let query = "INSERT INTO musics (id, name) VALUES (?, ?);";

        Box::pin(async move {
            sqlx::query::<Self::Database>(query)
                .bind(music.id.to_string())
                .bind(music.name)
                .execute(&pool)
                .await
                .map_err(crate::db::Error::Sqlx)?;
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
