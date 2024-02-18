use sqlx::{ColumnIndex, Pool, Row, Sqlite};
use std::{future::Future, pin::Pin};

use crate::db::{
    content::Image,
    Table,
};

pub struct ImageTable {
    pub pool: Pool<Sqlite>,
}

impl<'r, R> Table<'r, R> for ImageTable
where
    R: Row,
    &'r str: ColumnIndex<R>,
    String: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
    i64: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
{
    type Item = Image;
    type Database = Sqlite;

    fn get<Q: ToString + Send + 'static>(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Option<Self::Item>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM images where id = $1;";

        Box::pin(async move {
            if let Ok(music) = sqlx::query_as::<Self::Database, Image>(query)
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
        let query = "SELECT * FROM images where id = $1;";

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
        let query = "SELECT * FROM images;";

        Box::pin(async move {
            sqlx::query_as::<Self::Database, Self::Item>(query)
                .fetch_all(&pool)
                .await
                .unwrap()
        })
    }

    fn save(&self, image: Self::Item) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let pool = self.pool.clone();
        let query = "INSERT INTO images (id, file) VALUES (?, ?);";

        Box::pin(async move {
            sqlx::query::<Self::Database>(query)
                .bind(image.id.to_string())
                .bind(image.file.to_string())
                .execute(&pool)
                .await
                .unwrap();
        })
    }

    fn save_many(&self, _items: Vec<Self::Item>) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        todo!()
    }
}
