use serde::{Deserialize, Serialize};
use sqlx::{Row, ColumnIndex};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Music {
    pub name: String,
    pub id: Uuid,
}

#[derive(Debug, Clone)]
pub struct Image {
    pub file: Uuid,
    pub id: Uuid,
}

#[derive(Debug, Clone)]
pub struct Audio {
    pub file: Uuid,
    pub id: Uuid,
}

#[derive(Debug, Clone)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub location: String,
    pub size: u64,
    pub mime: String,
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


