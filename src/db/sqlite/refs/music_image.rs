use std::{future::Future, pin::Pin};

use crate::db::{
    content::{Image, Music},
    content_refs::MusicImageRef,
    Table, TableFetchWhereImage, TableFetchWhereMusic, TableMusicImageRef,
};
use sqlx::{Sqlite, SqlitePool};

#[derive(Clone)]
pub struct MusicImageTable {
    pub pool: SqlitePool,
}

impl<Q: ToString + Send + 'static> Table<Q> for MusicImageTable {
    type Item = MusicImageRef;
    type Database = Sqlite;

    fn get(&self, id: Q) -> Pin<Box<dyn Future<Output = Option<Self::Item>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_images where id = $1;";

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
        let query = "SELECT * FROM music_images where id = $1;";

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
        let query = "SELECT * FROM music_images;";

        Box::pin(async move {
            sqlx::query_as::<Self::Database, Self::Item>(query)
                .fetch_all(&pool)
                .await
                .unwrap()
        })
    }

    fn save(&self, music_image_ref: Self::Item) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let pool = self.pool.clone();
        let query = "INSERT INTO music_images (id, music, image) VALUES (?, ?, ?);";

        Box::pin(async move {
            sqlx::query::<Self::Database>(query)
                .bind(music_image_ref.id.to_string())
                .bind(music_image_ref.music.to_string())
                .bind(music_image_ref.image.to_string())
                .execute(&pool)
                .await
                .unwrap();
        })
    }

    fn save_many(&self, _items: Vec<Self::Item>) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        todo!()
    }
}

impl<Q: ToString + Send + 'static> TableFetchWhereImage<Q> for MusicImageTable {
    type ItemWhereImage = MusicImageRef;

    fn get_where_image(
        &self,
        image: Image,
    ) -> Pin<Box<dyn Future<Output = Option<Self::ItemWhereImage>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_images where image = $1;";
        let image_id = image.id.clone().to_string();

        Box::pin(async move {
            if let Ok(item) = sqlx::query_as::<sqlx::Sqlite, Self::ItemWhereImage>(query)
                .bind(image_id)
                .fetch_one(&pool)
                .await
            {
                Some(item)
            } else {
                None
            }
        })
    }

    fn get_where_image_id(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Option<Self::ItemWhereImage>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_images where image = $1;";

        Box::pin(async move {
            if let Ok(item) = sqlx::query_as::<sqlx::Sqlite, Self::ItemWhereImage>(query)
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
}

impl<Q: ToString + Send + 'static> TableFetchWhereMusic<Q> for MusicImageTable {
    type ItemWhereMusic = MusicImageRef;

    fn get_where_music(
        &self,
        music: Music,
    ) -> Pin<Box<dyn Future<Output = Option<Self::ItemWhereMusic>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_images WHERE music = $1;";
        let music_id = music.id.to_string();

        Box::pin(async move {
            if let Ok(item) = sqlx::query_as::<sqlx::Sqlite, Self::ItemWhereMusic>(query)
                .bind(music_id)
                .fetch_one(&pool)
                .await
            {
                Some(item)
            } else {
                None
            }
        })
    }

    fn get_where_music_id(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Option<Self::ItemWhereMusic>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_images WHERE music = $1;";

        Box::pin(async move {
            if let Ok(item) = sqlx::query_as::<sqlx::Sqlite, Self::ItemWhereMusic>(query)
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
}

impl TableMusicImageRef for MusicImageTable {}
