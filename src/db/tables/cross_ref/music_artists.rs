use std::str::FromStr;

use sqlx::{sqlite::SqlitePool, ColumnIndex, Executor, Row};
use uuid::Uuid;

use crate::db::tables::musics::Music;

#[derive(Clone)]
pub struct MusicArtistTable {
    pub pool: SqlitePool,
}

#[derive(Debug, Clone)]
pub struct MusicArtistRef {
    pub id: Uuid,
    pub image: Uuid,
    pub artist: Uuid,
}

impl MusicArtistRef {
    pub fn new(artist: &str, image: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            artist: Uuid::from_str(artist).unwrap(),
            image: Uuid::from_str(image).unwrap(),
        }
    }
}

impl<'r, R> sqlx::FromRow<'r, R> for MusicArtistRef
where
    R: Row,
    &'r str: ColumnIndex<R>,
    String: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
    i64: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
{
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let music: String = row.try_get("music")?;
        let artist: String = row.try_get("artist")?;

        Ok(Self {
            id: Uuid::parse_str(&id).unwrap(),
            artist: Uuid::parse_str(&artist).unwrap(),
            image: Uuid::parse_str(&music).unwrap(),
        })
    }
}

impl MusicArtistTable {
    pub async fn checks(pool: &SqlitePool) {
        let create_table = "CREATE TABLE IF NOT EXISTS music_artists (id varchar (36) PRIMARY KEY, music varchar(36) REFERENCES musics (id), artist varchar(36) REFERENCES artists (id));";
        pool.execute(sqlx::query(&create_table)).await.unwrap();
    }

    pub async fn new(pool: SqlitePool) -> Self {
        Self::checks(&pool).await;
        Self { pool }
    }

    pub async fn all(&self) -> Vec<MusicArtistRef> {
        let query = "SELECT * FROM music_artists;";

        sqlx::query_as::<_, MusicArtistRef>(query)
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    pub async fn get_from_music_id<T: ToString>(&self, music_id: T) -> Vec<MusicArtistRef> {
        let query = "SELECT * FROM music_artists WHERE music = $1;";

        sqlx::query_as::<_, MusicArtistRef>(query)
            .bind(music_id.to_string())
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    pub async fn get_from_music(&self, music: &Music) -> Vec<MusicArtistRef> {
        self.get_from_music_id(music.id.to_string()).await
        // let query = "SELECT * FROM music_artists WHERE music = $1;";

        // sqlx::query_as::<_, MusicArtistRef>(query)
        //     .bind(music.get_primary_key())
        //     .fetch_all(&self.pool)
        //     .await
        //     .unwrap()
    }

    pub async fn save(&self, music_image_ref: MusicArtistRef) {
        let query = "INSERT INTO music_artists (id, music, image) VALUES (?, ?, ?);";

        let row = sqlx::query(query)
            .bind(music_image_ref.id.to_string())
            .bind(music_image_ref.artist.to_string())
            .bind(music_image_ref.image.to_string())
            .execute(&self.pool)
            .await
            .unwrap();

        println!("save row: {:?}", row);
    }
}
