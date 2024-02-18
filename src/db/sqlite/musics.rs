use std::{future::Future, pin::Pin};

use sqlx::{Pool, Sqlite};

use crate::db::{content::Music, Table};

pub struct MusicTable {
    pub pool: Pool<Sqlite>,
}

impl Table for MusicTable {
    type Item = Music;

    fn get<Q: ToString + Send + 'static>(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Option<Self::Item>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM musics where id = $1;";

        Box::pin(async move {
            if let Ok(music) = sqlx::query_as::<_, Music>(query)
                .bind(id.to_string())
                .fetch_one(&pool)
                .await
            {
                Some(music)
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
        let query = "SELECT * FROM musics where id = $1;";

        Box::pin(async move {
            sqlx::query_as::<_, Music>(query)
                .bind(id.to_string())
                .fetch_all(&pool)
                .await
                .unwrap()
        })
    }

    fn get_all(&self) -> Pin<Box<dyn Future<Output = Vec<Self::Item>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM musics;";

        Box::pin(async move {
            sqlx::query_as::<_, Music>(query)
                .fetch_all(&pool)
                .await
                .unwrap()
        })
    }

    fn save(&self, music: Self::Item) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let pool = self.pool.clone();
        let query = "INSERT INTO musics (id, name) VALUES (?, ?);";

        Box::pin(async move {
            sqlx::query(query)
                .bind(music.id.to_string())
                .bind(music.name)
                .execute(&pool)
                .await
                .unwrap();
        })
    }

    fn save_many(&self, _items: Vec<Self::Item>) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        todo!()
    }
}
