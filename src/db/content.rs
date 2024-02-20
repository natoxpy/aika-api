use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{ColumnIndex, Row};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Music {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub file: Uuid,
    pub id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub id: Uuid,
    pub name: String,
    pub cover: Uuid,
    pub released: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audio {
    pub id: Uuid,
    pub file: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    i64: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
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

impl<'r, R> sqlx::FromRow<'r, R> for Image
where
    R: Row,
    &'r str: ColumnIndex<R>,
    String: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
    i64: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
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
        let avatar: Option<String> = row.try_get("avatar")?;

        Ok(Self {
            name,
            avatar,
            id: Uuid::parse_str(&id).unwrap(),
        })
    }
}

impl<'r, R> sqlx::FromRow<'r, R> for Audio
where
    R: Row,
    &'r str: ColumnIndex<R>,
    String: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
    i64: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
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

impl<'r, R> sqlx::FromRow<'r, R> for Album
where
    R: Row,
    &'r str: ColumnIndex<R>,
    String: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
    i64: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
{
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let cover: String = row.try_get("cover")?;

        // TODO: Implemented release date for cover
        // let released: String = row.try_get("released")?;

        Ok(Self {
            id: Uuid::parse_str(&id).unwrap(),
            name,
            cover: Uuid::parse_str(&cover).unwrap(),
            released: None,
        })
    }
}
