use sqlx::{sqlite::SqlitePool, ColumnIndex, Executor, Row};
use uuid::Uuid;

use super::QueryPrimaryKey;

#[derive(Clone)]
pub struct ArtistTable {
    pub pool: SqlitePool,
}

#[derive(Debug, Clone)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
}

impl Artist {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            id: Uuid::new_v4(),
        }
    }
}

impl<'r, R> sqlx::FromRow<'r, R> for Artist
where
    R: Row,
    &'r str: ColumnIndex<R>,
    String: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
    i64: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
{
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let name: String = row.try_get("name")?;

        Ok(Self {
            id: Uuid::parse_str(&id).unwrap(),
            name,
        })
    }
}

impl ArtistTable {
    pub async fn checks(pool: &SqlitePool) {
        let create_table = "CREATE TABLE IF NOT EXISTS artists (id varchar (36) PRIMARY KEY, name varchar(255));";
        pool.execute(sqlx::query(&create_table)).await.unwrap();
    }

    pub async fn new(pool: SqlitePool) -> Self {
        Self::checks(&pool).await;
        Self { pool }
    }

    pub async fn get<T: QueryPrimaryKey>(&self, id: T) -> Option<Artist> {
        let query = "SELECT * FROM artists WHERE id = $1;";

        if let Ok(img) = sqlx::query_as::<_, Artist>(query)
            .bind(id.get_primary_key())
            .fetch_one(&self.pool)
            .await
        {
            Some(img)
        } else {
            None
        }
    }

    pub async fn all(&self) {
        let query = "SELECT * FROM artists;";

        let files = sqlx::query_as::<_, Artist>(query)
            .fetch_all(&self.pool)
            .await
            .unwrap();

        println!("{:?}", files);
    }

    pub async fn save(&self, file: Artist) {
        let query = "INSERT INTO artists (id, name) VALUES (?, ?, ?, ?, ?);";

        let row = sqlx::query(query)
            .bind(file.id.to_string())
            .bind(file.name)
            .execute(&self.pool)
            .await
            .unwrap();

        println!("save row: {:?}", row);
    }
}
