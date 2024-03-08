use serde::{Deserialize, Serialize};
use sqlx::{ColumnIndex, Row};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AlbumArtistRef {
    pub id: Uuid,
    pub artist: Uuid,
    pub album: Uuid,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MusicAlbumRef {
    pub id: Uuid,
    pub music: Uuid,
    pub album: Uuid,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MusicArtistRef {
    pub id: Uuid,
    pub music: Uuid,
    pub artist: Uuid,
}

#[derive(Debug, Clone)]
pub struct MusicAudioRef {
    pub id: Uuid,
    pub music: Uuid,
    pub audio: Uuid,
}

#[derive(Debug, Clone)]
pub struct MusicImageRef {
    pub id: Uuid,
    pub image: Uuid,
    pub music: Uuid,
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
            music: Uuid::parse_str(&music).unwrap(),
        })
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

impl<'r, R> sqlx::FromRow<'r, R> for MusicAlbumRef
where
    R: Row,
    &'r str: ColumnIndex<R>,
    String: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
    i64: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
{
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let music: String = row.try_get("music")?;
        let album: String = row.try_get("album")?;

        Ok(Self {
            id: Uuid::parse_str(&id).unwrap(),
            music: Uuid::parse_str(&music).unwrap(),
            album: Uuid::parse_str(&album).unwrap(),
        })
    }
}

impl<'r, R> sqlx::FromRow<'r, R> for AlbumArtistRef
where
    R: Row,
    &'r str: ColumnIndex<R>,
    String: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
    i64: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
{
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let artist: String = row.try_get("artist")?;
        let album: String = row.try_get("album")?;

        Ok(Self {
            id: Uuid::parse_str(&id).unwrap(),
            artist: Uuid::parse_str(&artist).unwrap(),
            album: Uuid::parse_str(&album).unwrap(),
        })
    }
}
