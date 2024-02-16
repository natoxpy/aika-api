use sqlx::{sqlite::SqlitePool, ColumnIndex, Executor, Row};
use uuid::Uuid;

#[derive(Clone)]
pub struct FileTable {
    pub pool: SqlitePool,
}

#[derive(Debug, Clone)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub location: String,
    pub size: u64,
    pub mime: String,
}

impl File {
    pub fn new(name: &str, location: String, size: u64, mime: String) -> Self {
        Self {
            name: name.to_string(),
            location,
            size,
            mime,
            id: Uuid::new_v4(),
        }
    }
}

impl<'r, R> sqlx::FromRow<'r, R> for File
where
    R: Row,
    &'r str: ColumnIndex<R>,
    String: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
    i64: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
{
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let location: String = row.try_get("location")?;
        let size: i64 = row.try_get("size")?;
        let filetype: String = row.try_get("mime")?;

        Ok(Self {
            id: Uuid::parse_str(&id).unwrap(),
            name,
            location,
            size: size as u64,
            mime: filetype,
        })
    }
}

impl FileTable {
    pub async fn checks(pool: &SqlitePool) {
        let create_table = "CREATE TABLE IF NOT EXISTS files (id INTEGER PRIMARY KEY, location TEXT, size INTEGER, mime varchar(16), name varchar(255));";
        pool.execute(sqlx::query(&create_table)).await.unwrap();
    }

    pub async fn new(pool: SqlitePool) -> Self {
        Self::checks(&pool).await;
        Self { pool }
    }

    pub async fn get<T: ToString>(&self, id: T) -> Option<File> {
        let query = "SELECT * FROM files WHERE id = $1;";

        if let Ok(img) = sqlx::query_as::<_, File>(query)
            .bind(id.to_string())
            .fetch_one(&self.pool)
            .await
        {
            Some(img)
        } else {
            None
        }
    }

    pub async fn all(&self) {
        let query = "SELECT * FROM files;";

        let files = sqlx::query_as::<_, File>(query)
            .fetch_all(&self.pool)
            .await
            .unwrap();

        println!("{:?}", files);
    }

    pub async fn save(&self, file: File) {
        let query = "INSERT INTO files (id, location, size, mime, name) VALUES (?, ?, ?, ?, ?);";

        let row = sqlx::query(query)
            .bind(file.id.to_string())
            .bind(file.location)
            .bind(file.size as i64)
            .bind(file.mime)
            .bind(file.name)
            .execute(&self.pool)
            .await
            .unwrap();

        println!("save row: {:?}", row);
    }
}
