use std::str::FromStr;

use sqlx::{sqlite::SqlitePool, ColumnIndex, Executor, Row};
use uuid::Uuid;

use crate::db::tables::musics::Music;

#[derive(Clone)]
pub struct MusicImageTable {
    pub pool: SqlitePool,
}

#[derive(Debug, Clone)]
pub struct MusicImageRef {
    pub id: Uuid,
    pub image: Uuid,
    pub music: Uuid,
}

impl MusicImageRef {
    pub fn new(music: &str, image: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            music: Uuid::from_str(music).unwrap(),
            image: Uuid::from_str(image).unwrap(),
        }
    }
}

impl<'r, R> sqlx::FromRow<'r, R> for MusicImageRef
where
    R: Row,
    &'r str: ColumnIndex<R>,
    String: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
    i64: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
{
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let music: String = row.try_get("music")?;
        let image: String = row.try_get("image")?;

        Ok(Self {
            id: Uuid::parse_str(&id).unwrap(),
            music: Uuid::parse_str(&music).unwrap(),
            image: Uuid::parse_str(&image).unwrap(),
        })
    }
}

impl MusicImageTable {
    pub async fn checks(pool: &SqlitePool) {
        let create_table = "CREATE TABLE IF NOT EXISTS music_images (id varchar(36) PRIMARY KEY, image varchar(36) REFERENCES images (id), music varchar(36) REFERENCES musics (id));";
        pool.execute(sqlx::query(&create_table)).await.unwrap();
    }

    pub async fn new(pool: SqlitePool) -> Self {
        Self::checks(&pool).await;
        Self { pool }
    }

    pub async fn all(&self) -> Vec<MusicImageRef> {
        let query = "SELECT * FROM music_images;";

        sqlx::query_as::<_, MusicImageRef>(query)
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    pub async fn get_from_music_id<T: ToString>(&self, music_id: T) -> Vec<MusicImageRef> {
        let query = "SELECT * FROM music_images WHERE music = $1;";

        sqlx::query_as::<_, MusicImageRef>(query)
            .bind(music_id.to_string())
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    pub async fn get_from_music(&self, music: &Music) -> Vec<MusicImageRef> {
        self.get_from_music_id(music.id.to_string()).await
        // let query = "SELECT * FROM music_images WHERE music = $1;";

        // sqlx::query_as::<_, MusicImageRef>(query)
        //     .bind(music.get_primary_key())
        //     .fetch_all(&self.pool)
        //     .await
        //     .unwrap()
    }

    pub async fn save(&self, music_image_ref: MusicImageRef) {
        let query = "INSERT INTO music_images (id, music, image) VALUES (?, ?, ?);";

        let row = sqlx::query(query)
            .bind(music_image_ref.id.to_string())
            .bind(music_image_ref.music.to_string())
            .bind(music_image_ref.image.to_string())
            .execute(&self.pool)
            .await
            .unwrap();

        println!("save row: {:?}", row);
    }
}
