use sqlx::{sqlite::SqlitePool, ColumnIndex, Executor, Row};
use uuid::Uuid;

use super::QueryPrimaryKey;

#[derive(Clone)]
pub struct ImageTable {
    pub pool: SqlitePool,
}

#[derive(Debug, Clone)]
pub struct Image {
    pub file: Uuid,
    pub id: Uuid,
}

impl Image {
    pub fn new(file_id: &str) -> Self {
        Self {
            file: Uuid::parse_str(&file_id).unwrap(),
            id: Uuid::new_v4(),
        }
    }
}

impl<'r, R> sqlx::FromRow<'r, R> for Image
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

impl ImageTable {
    async fn checks(pool: &SqlitePool) {
        let create_table =
            "CREATE TABLE IF NOT EXISTS images (id varchar (36) PRIMARY KEY, file varchar (36) REFERENCES files (id));";
        pool.execute(sqlx::query(&create_table)).await.unwrap();
    }

    pub async fn new(pool: SqlitePool) -> Self {
        Self::checks(&pool).await;

        Self { pool }
    }

    pub async fn get<T: QueryPrimaryKey>(&self, id: T) -> Option<Image> {
        let query = "SELECT * FROM images WHERE id = $1;";

        if let Ok(img) = sqlx::query_as::<_, Image>(query)
            .bind(id.get_primary_key())
            .fetch_one(&self.pool)
            .await
        {
            Some(img)
        } else {
            None
        }
    }

    pub async fn all(&self) -> Vec<Image> {
        let query = "SELECT * FROM images;";

        sqlx::query_as::<_, Image>(query)
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    pub async fn save(&self, music: Image) {
        let query = "INSERT INTO images (id, file) VALUES (?, ?);";
        let row = sqlx::query(query)
            .bind(music.id.to_string())
            .bind(music.file.to_string())
            .execute(&self.pool)
            .await
            .unwrap();

        println!("save row: {:?}", row);
    }
}
