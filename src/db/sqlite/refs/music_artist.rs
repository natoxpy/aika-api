use crate::db::{
    content::{Artist, Music},
    content_refs::MusicArtistRef,
    Table, TableFetchWhereArtist, TableFetchWhereMusic, TableMusicArtistRef,
};
use sqlx::SqlitePool;
use std::{future::Future, pin::Pin};

use sqlx::Sqlite;

#[derive(Clone)]
pub struct MusicArtistTable {
    pub pool: SqlitePool,
}

impl<Q: ToString + Send + 'static> Table<Q> for MusicArtistTable {
    type Item = MusicArtistRef;
    type Database = Sqlite;

    fn get(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Item, crate::db::Error>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_artists where id = $1;";

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
        let query = "SELECT * FROM music_artists where id = $1;";

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
        let query = "SELECT * FROM music_artists;";

        Box::pin(async move {
            sqlx::query_as::<Self::Database, Self::Item>(query)
                .fetch_all(&pool)
                .await
                .map_err(crate::db::Error::Sqlx)
        })
    }

    fn save(
        &self,
        music_image_ref: Self::Item,
    ) -> Pin<Box<dyn Future<Output = Result<(), crate::db::Error>> + Send>> {
        let pool = self.pool.clone();
        let query = "INSERT INTO music_artists (id, music, artist) VALUES (?, ?, ?);";

        Box::pin(async move {
            sqlx::query::<Self::Database>(query)
                .bind(music_image_ref.id.to_string())
                .bind(music_image_ref.music.to_string())
                .bind(music_image_ref.artist.to_string())
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

impl<Q: ToString + Send + 'static> TableFetchWhereArtist<Q> for MusicArtistTable {
    type ItemWhereArtist = MusicArtistRef;

    fn get_where_artist(
        &self,
        artist: Artist,
    ) -> Pin<Box<dyn Future<Output = Result<Self::ItemWhereArtist, crate::db::Error>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_artists where artist = $1;";
        let artist_id = artist.id.clone().to_string();

        Box::pin(async move {
            sqlx::query_as::<sqlx::Sqlite, Self::ItemWhereArtist>(query)
                .bind(artist_id)
                .fetch_one(&pool)
                .await
                .map_err(crate::db::Error::Sqlx)
        })
    }

    fn get_where_artist_id(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Result<Self::ItemWhereArtist, crate::db::Error>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_artists where artist = $1;";

        Box::pin(async move {
            sqlx::query_as::<sqlx::Sqlite, Self::ItemWhereArtist>(query)
                .bind(id.to_string())
                .fetch_one(&pool)
                .await
                .map_err(crate::db::Error::Sqlx)
        })
    }
}

impl<Q: ToString + Send + 'static> TableFetchWhereMusic<Q> for MusicArtistTable {
    type ItemWhereMusic = MusicArtistRef;

    fn get_where_music(
        &self,
        music: Music,
    ) -> Pin<Box<dyn Future<Output = Result<Self::ItemWhereMusic, crate::db::Error>> + Send>> {
        self.get_where_music_id(music.id)
    }

    fn get_where_music_id(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Result<Self::ItemWhereMusic, crate::db::Error>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_artists where music = $1;";

        Box::pin(async move {
            sqlx::query_as::<sqlx::Sqlite, Self::ItemWhereMusic>(query)
                .bind(id.to_string())
                .fetch_one(&pool)
                .await
                .map_err(crate::db::Error::Sqlx)
        })
    }

    fn get_all_where_music(
        &self,
        music: Music,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Self::ItemWhereMusic>, crate::db::Error>> + Send>>
    {
        self.get_all_where_music_id(music.id)
    }

    fn get_all_where_music_id(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Self::ItemWhereMusic>, crate::db::Error>> + Send>>
    {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_artists where music = $1;";

        Box::pin(async move {
            sqlx::query_as::<sqlx::Sqlite, Self::ItemWhereMusic>(query)
                .bind(id.to_string())
                .fetch_all(&pool)
                .await
                .map_err(crate::db::Error::Sqlx)
        })
    }
}

impl TableMusicArtistRef for MusicArtistTable {}
