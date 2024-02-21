use crate::db::{
    content::{Album, Artist},
    content_refs::AlbumArtistRef,
    Table, TableAlbumArtistRef, TableFetchWhereAlbum, TableFetchWhereArtist,
};
use sqlx::SqlitePool;
use std::{future::Future, pin::Pin};

use sqlx::Sqlite;

#[derive(Clone)]
pub struct AlbumArtistTable {
    pub pool: SqlitePool,
}

impl<Q: ToString + Send + 'static> Table<Q> for AlbumArtistTable {
    type Item = AlbumArtistRef;
    type Database = Sqlite;

    fn get(&self, id: Q) -> Pin<Box<dyn Future<Output = Option<Self::Item>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM album_artists where id = $1;";

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
        let query = "SELECT * FROM album_artists where id = $1;";

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
        let query = "SELECT * FROM album_artists;";

        Box::pin(async move {
            sqlx::query_as::<Self::Database, Self::Item>(query)
                .fetch_all(&pool)
                .await
                .unwrap()
        })
    }

    fn save(&self, album_artist_ref: Self::Item) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let pool = self.pool.clone();
        let query = "INSERT INTO album_artists (id, artist, albums) VALUES (?, ?);";

        Box::pin(async move {
            sqlx::query::<Self::Database>(query)
                .bind(album_artist_ref.id.to_string())
                .bind(album_artist_ref.artist.to_string())
                .bind(album_artist_ref.album.to_string())
                .execute(&pool)
                .await
                .unwrap();
        })
    }

    fn save_many(&self, _items: Vec<Self::Item>) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        todo!()
    }
}

impl<Q: ToString + Send + 'static> TableFetchWhereAlbum<Q> for AlbumArtistTable {
    type ItemWhereAlbum = AlbumArtistRef;

    fn get_where_album(
        &self,
        album: Album,
    ) -> Pin<Box<dyn Future<Output = Option<Self::ItemWhereAlbum>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM album_artists where album = $1;";
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
        let query = "SELECT * FROM album_artists where album = $1;";

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

impl<Q: ToString + Send + 'static> TableFetchWhereArtist<Q> for AlbumArtistTable {
    type ItemWhereArtist = AlbumArtistRef;

    fn get_where_artist(
        &self,
        artist: Artist,
    ) -> Pin<Box<dyn Future<Output = Option<Self::ItemWhereArtist>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM album_artists where artist = $1;";
        let artist_id = artist.id.to_string();

        Box::pin(async move {
            if let Ok(item) = sqlx::query_as::<sqlx::Sqlite, Self::ItemWhereArtist>(query)
                .bind(artist_id)
                .fetch_one(&pool)
                .await
            {
                Some(item)
            } else {
                None
            }
        })
    }

    fn get_where_artist_id(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Option<Self::ItemWhereArtist>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM album_artists where artist = $1;";

        Box::pin(async move {
            if let Ok(item) = sqlx::query_as::<sqlx::Sqlite, Self::ItemWhereArtist>(query)
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

impl TableAlbumArtistRef for AlbumArtistTable {}
