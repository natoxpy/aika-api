use sqlx::{sqlite::SqlitePool, ColumnIndex, Executor, Row};
use uuid::Uuid;

#[derive(Clone)]
pub struct MusicAudioTable {
    pub pool: SqlitePool,
}

#[derive(Debug, Clone)]
pub struct MusicAudioRef {
    pub id: Uuid,
    pub music: Uuid,
    pub audio: Uuid,
}

impl MusicAudioRef {
    pub fn new(music: String, audio: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            music: Uuid::parse_str(&music).unwrap(),
            audio: Uuid::parse_str(&audio).unwrap(),
        }
    }
}

impl<'r, R> sqlx::FromRow<'r, R> for MusicAudioRef
where
    R: Row,
    &'r str: ColumnIndex<R>,
    String: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
{
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let music: String = row.try_get("music")?;
        let audio: String = row.try_get("audio")?;

        Ok(Self {
            id: Uuid::parse_str(&id).unwrap(),
            music: Uuid::parse_str(&music).unwrap(),
            audio: Uuid::parse_str(&audio).unwrap(),
        })
    }
}

impl MusicAudioTable {
    pub async fn checks(pool: &SqlitePool) {
        let create_table = "CREATE TABLE IF NOT EXISTS music_audios (id varchar (36) PRIMARY KEY, music varchar (36) REFERENCES musics (id), audio varchar (36) REFERENCES audios (id));";
        pool.execute(sqlx::query(&create_table)).await.unwrap();
    }

    pub async fn new(pool: SqlitePool) -> Self {
        Self::checks(&pool).await;
        Self { pool }
    }

    pub async fn get_from_music_id<T: ToString>(&self, music_id: T) -> Vec<MusicAudioRef> {
        let query = "SELECT * FROM music_audios WHERE music = $1;";

        sqlx::query_as::<_, MusicAudioRef>(query)
            .bind(music_id.to_string())
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    pub async fn all(&self) {
        let query = "SELECT * FROM music_audios;";

        let musicfiles = sqlx::query_as::<_, MusicAudioRef>(query)
            .fetch_all(&self.pool)
            .await
            .unwrap();

        println!("{:?}", musicfiles);
    }

    pub async fn save(&self, musicfile: MusicAudioRef) {
        let query = "INSERT INTO music_audios (id, music, audio) VALUES (?, ?, ?);";

        let row = sqlx::query(query)
            .bind(musicfile.id.to_string())
            .bind(musicfile.music.to_string())
            .bind(musicfile.audio.to_string())
            .execute(&self.pool)
            .await
            .unwrap();

        println!("save row: {:?}", row);
    }
}
