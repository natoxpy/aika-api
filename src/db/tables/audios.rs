use sqlx::{sqlite::SqlitePool, ColumnIndex, Executor, Row};
use uuid::Uuid;

use super::QueryPrimaryKey;

#[derive(Clone)]
pub struct AudioTable {
    pub pool: SqlitePool,
}

#[derive(Debug, Clone)]
pub struct Audio {
    pub file: Uuid,
    pub id: Uuid,
}

impl Audio {
    pub fn new(file_id: &str) -> Self {
        Self {
            file: Uuid::parse_str(&file_id).unwrap(),
            id: Uuid::new_v4(),
        }
    }
}

impl<'r, R> sqlx::FromRow<'r, R> for Audio
where
    R: Row,
    &'r str: ColumnIndex<R>,
    String: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
{
    #[inline]
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let file: String = row.try_get("file")?;

        Ok(Self {
            file: Uuid::parse_str(&file).unwrap(),
            id: Uuid::parse_str(&id).unwrap(),
        })
    }
}

impl AudioTable {
    async fn checks(pool: &SqlitePool) {
        let create_table =
            "CREATE TABLE IF NOT EXISTS audios (id varchar (36) PRIMARY KEY, file varchar (36) REFERENCES files (id));";
        pool.execute(sqlx::query(&create_table)).await.unwrap();
    }

    pub async fn new(pool: SqlitePool) -> Self {
        Self::checks(&pool).await;

        Self { pool }
    }

    pub async fn get<T: QueryPrimaryKey>(&self, id: T) -> Option<Audio> {
        let query = "SELECT * FROM audios WHERE id = $1;";

        if let Ok(audio) = sqlx::query_as::<_, Audio>(query)
            .bind(id.get_primary_key())
            .fetch_one(&self.pool)
            .await
        {
            Some(audio)
        } else {
            None
        }
    }

    pub async fn all(&self) -> Vec<Audio> {
        let query = "SELECT * FROM audios;";

        sqlx::query_as::<_, Audio>(query)
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    pub async fn save(&self, music: Audio) {
        let query = "INSERT INTO audios (id, file) VALUES (?, ?);";
        let row = sqlx::query(query)
            .bind(music.id.to_string())
            .bind(music.file.to_string())
            .execute(&self.pool)
            .await
            .unwrap();

        println!("save row: {:?}", row);
    }
}
