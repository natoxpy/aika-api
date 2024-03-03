use std::{future::Future, pin::Pin};

use crate::db::{
    content::{Audio, Music},
    content_refs::MusicAudioRef,
    Table, TableFetchWhereAudio, TableFetchWhereMusic, TableMusicAudioRef,
};
use sqlx::{Sqlite, SqlitePool};

#[derive(Clone)]
pub struct MusicAudioTable {
    pub pool: SqlitePool,
}

impl<Q: ToString + Send + 'static> Table<Q> for MusicAudioTable {
    type Item = MusicAudioRef;
    type Database = Sqlite;

    fn get(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Item, crate::db::Error>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_audios where id = $1;";

        Box::pin(async move {
            sqlx::query_as::<Self::Database, Self::Item>(query)
                .bind(id.to_string())
                .fetch_one(&pool)
                .await
                .map_err(|err| crate::db::Error::Sqlx(err))
        })
    }

    fn get_many(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Self::Item>, crate::db::Error>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_audios where id = $1;";

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
        let query = "SELECT * FROM music_audios;";

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
        let query = "INSERT INTO music_audios (id, music, audio) VALUES (?, ?, ?);";

        Box::pin(async move {
            sqlx::query::<Self::Database>(query)
                .bind(music_image_ref.id.to_string())
                .bind(music_image_ref.music.to_string())
                .bind(music_image_ref.audio.to_string())
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

impl<Q: ToString + Send + 'static> TableFetchWhereAudio<Q> for MusicAudioTable {
    type ItemWhereAudio = MusicAudioRef;

    fn get_where_audio(
        &self,
        audio: Audio,
    ) -> Pin<Box<dyn Future<Output = Result<Self::ItemWhereAudio, crate::db::Error>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_audios where audio = $1;";
        let audio_id = audio.id.clone().to_string();

        Box::pin(async move {
            sqlx::query_as::<sqlx::Sqlite, Self::ItemWhereAudio>(query)
                .bind(audio_id)
                .fetch_one(&pool)
                .await
                .map_err(|err| crate::db::Error::Sqlx(err))
        })
    }

    fn get_where_audio_id(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Result<Self::ItemWhereAudio, crate::db::Error>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_audios where audio = $1;";

        Box::pin(async move {
            sqlx::query_as::<sqlx::Sqlite, Self::ItemWhereAudio>(query)
                .bind(id.to_string())
                .fetch_one(&pool)
                .await
                .map_err(|err| crate::db::Error::Sqlx(err))
        })
    }
}

impl<Q: ToString + Send + 'static> TableFetchWhereMusic<Q> for MusicAudioTable {
    type ItemWhereMusic = MusicAudioRef;

    fn get_where_music(
        &self,
        music: Music,
    ) -> Pin<Box<dyn Future<Output = Result<Self::ItemWhereMusic, crate::db::Error>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_audios where music = $1;";
        let music_id = music.id.to_string();

        Box::pin(async move {
            sqlx::query_as::<sqlx::Sqlite, Self::ItemWhereMusic>(query)
                .bind(music_id)
                .fetch_one(&pool)
                .await
                .map_err(|err| crate::db::Error::Sqlx(err))
        })
    }

    fn get_where_music_id(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Result<Self::ItemWhereMusic, crate::db::Error>> + Send>> {
        let pool = self.pool.clone();
        let query = "SELECT * FROM music_audios where music = $1;";

        Box::pin(async move {
            sqlx::query_as::<sqlx::Sqlite, Self::ItemWhereMusic>(query)
                .bind(id.to_string())
                .fetch_one(&pool)
                .await
                .map_err(|err| crate::db::Error::Sqlx(err))
        })
    }
}

impl TableMusicAudioRef for MusicAudioTable {}