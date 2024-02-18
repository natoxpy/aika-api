use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, ColumnIndex, Executor, Row};
use uuid::Uuid;

#[derive(Clone)]
pub struct MusicTable {
    pub pool: SqlitePool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Music {
    pub name: String,
    pub id: Uuid,
}

impl Music {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            id: Uuid::new_v4(),
        }
    }
}

impl<'r, R> sqlx::FromRow<'r, R> for Music
where
    R: Row,
    &'r str: ColumnIndex<R>,
    String: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
{
    #[inline]
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let name: String = row.try_get("name")?;

        Ok(Self {
            name,
            id: Uuid::parse_str(&id).unwrap(),
        })
    }
}

impl MusicTable {
    async fn checks(pool: &SqlitePool) {
        let create_table =
            "CREATE TABLE IF NOT EXISTS musics (id varchar (36) PRIMARY KEY, name varchar (255));";
        pool.execute(sqlx::query(&create_table)).await.unwrap();
    }

    pub async fn new(pool: SqlitePool) -> Self {
        Self::checks(&pool).await;

        Self { pool }
    }

    pub async fn get<T: ToString>(&self, id: T) -> Option<Music> {
        let query = "SELECT * FROM musics WHERE id = $1;";

        if let Ok(img) = sqlx::query_as::<_, Music>(query)
            .bind(id.to_string())
            .fetch_one(&self.pool)
            .await
        {
            Some(img)
        } else {
            None
        }
    }

    pub async fn all(&self) -> Vec<Music> {
        let query = "SELECT * FROM musics;";
        sqlx::query_as::<_, Music>(query)
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    pub async fn save(&self, music: Music) {
        let query = "INSERT INTO musics (id, name) VALUES (?, ?);";
        let row = sqlx::query(query)
            .bind(music.id.to_string())
            .bind(music.name)
            .execute(&self.pool)
            .await
            .unwrap();

        println!("save row: {:?}", row);
    }
}
