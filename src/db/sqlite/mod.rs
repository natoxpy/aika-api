use self::{
    artists::ArtistTable, audios::AudioTable, files::FileTable, images::ImageTable,
    musics::MusicTable,
};
use super::Tables;

pub mod albums;
pub mod artists;
pub mod audios;
pub mod files;
pub mod images;
pub mod musics;

pub struct SqliteTables {
    pub pool: sqlx::Pool<sqlx::Sqlite>,
    pub music_table: MusicTable,
    pub image_table: ImageTable,
    pub artist_table: ArtistTable,
    pub audio_table: AudioTable,
    pub file_table: FileTable,
}

impl Tables for SqliteTables {
    type Database = sqlx::Sqlite;

    fn new(pool: sqlx::Pool<Self::Database>) -> Box<dyn Tables<Database = sqlx::Sqlite>>
    where
        Self: Sized,
    {
        Box::new(Self {
            pool: pool.clone(),
            music_table: MusicTable { pool: pool.clone() },
            image_table: ImageTable { pool: pool.clone() },
            artist_table: ArtistTable { pool: pool.clone() },
            audio_table: AudioTable { pool: pool.clone() },
            file_table: FileTable { pool: pool.clone() },
        })
    }

    fn music(
        &self,
    ) -> Box<&dyn super::Table<Item = super::content::Music, Database = Self::Database>> {
        Box::new(&self.music_table)
    }

    fn image(
        &self,
    ) -> Box<&dyn super::Table<Item = super::content::Image, Database = Self::Database>> {
        Box::new(&self.image_table)
    }

    fn artists(
        &self,
    ) -> Box<&dyn super::Table<Item = super::content::Artist, Database = Self::Database>> {
        Box::new(&self.artist_table)
    }

    fn audio(
        &self,
    ) -> Box<&dyn super::Table<Item = super::content::Audio, Database = Self::Database>> {
        Box::new(&self.audio_table)
    }

    fn file(
        &self,
    ) -> Box<&dyn super::Table<Item = super::content::File, Database = Self::Database>> {
        todo!()
    }
}
