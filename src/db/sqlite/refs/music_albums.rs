use crate::db::{
    content::{Album, Music},
    content_refs::MusicAlbumRef,
    Table, TableFetchWhereAlbum, TableFetchWhereMusic, TableMusicAlbumRef,
};
use sqlx::SqlitePool;
use std::{future::Future, pin::Pin};

use sqlx::Sqlite;

#[derive(Clone)]
pub struct MusicAlbumTable {
    pub pool: SqlitePool,
}

impl<Q: ToString + Send + 'static> Table<Q> for MusicAlbumTable {
    type Item = MusicAlbumRef;
    type Database = Sqlite;

    fn get(&self, id: Q) -> Pin<Box<dyn Future<Output = Option<Self::Item>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_albums where id = $1;";

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
        let query = "SELECT * FROM music_albums where id = $1;";

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
        let query = "SELECT * FROM music_albums;";

        Box::pin(async move {
            sqlx::query_as::<Self::Database, Self::Item>(query)
                .fetch_all(&pool)
                .await
                .map_err(|err| crate::db::Error::Sqlx(err))
        })
    }

    fn save(
        &self,
        music_image_ref: Self::Item,
    ) -> Pin<Box<dyn Future<Output = Result<(), crate::db::Error>> + Send>> {
        let pool = self.pool.clone();
        let query = "INSERT INTO music_albums (id, music, album) VALUES (?, ?, ?);";

        Box::pin(async move {
            sqlx::query::<Self::Database>(query)
                .bind(music_image_ref.id.to_string())
                .bind(music_image_ref.music.to_string())
                .bind(music_image_ref.album.to_string())
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

impl<Q: ToString + Send + 'static> TableFetchWhereAlbum<Q> for MusicAlbumTable {
    type ItemWhereAlbum = MusicAlbumRef;

    fn get_where_album(
        &self,
        album: Album,
    ) -> Pin<Box<dyn Future<Output = Option<Self::ItemWhereAlbum>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_albums where album = $1;";
        let album_id = album.id.clone().to_string();

        Box::pin(async move {
            if let Ok(item) = sqlx::query_as::<sqlx::Sqlite, Self::ItemWhereAlbum>(query)
                .bind(album_id)
                .fetch_one(&pool)
                .await
            {
                Some(item)
            } else {
                None
            }
        })
    }

    fn get_where_album_id(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Option<Self::ItemWhereAlbum>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_albums where album = $1;";

        Box::pin(async move {
            if let Ok(item) = sqlx::query_as::<sqlx::Sqlite, Self::ItemWhereAlbum>(query)
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

impl<Q: ToString + Send + 'static> TableFetchWhereMusic<Q> for MusicAlbumTable {
    type ItemWhereMusic = MusicAlbumRef;

    fn get_where_music(
        &self,
        music: Music,
    ) -> Pin<Box<dyn Future<Output = Option<Self::ItemWhereMusic>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_albums where music = $1;";
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
        let query = "SELECT * FROM music_albums where music = $1;";

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

impl TableMusicAlbumRef for MusicAlbumTable {}
