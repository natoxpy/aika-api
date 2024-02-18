use crate::db::{
    sqlite::SqliteTables,
    tables::{
        artist::ArtistTable,
        audios::AudioTable,
        cross_ref::music_images::MusicImageTable,
        cross_ref::{music_artists::MusicArtistTable, music_audios::MusicAudioTable},
        files::FileTable,
        images::ImageTable,
        musics::MusicTable,
    },
    Tables,
};

pub const FILES: &'static str = "/home/natoxpy/.aika";

pub struct DBState {
    pub music_table: MusicTable,
    pub file_table: FileTable,
    pub audio_table: AudioTable,
    pub artist_table: ArtistTable,
    pub image_table: ImageTable,
    pub music_audio_table: MusicAudioTable,
    pub music_artist_table: MusicArtistTable,
    pub music_image_table: MusicImageTable,
}

pub struct DB<T: sqlx::Database = sqlx::Sqlite> {
    pub tables: Box<dyn Tables<Database = T>>,
}

impl<T: sqlx::Database> DB<T> {
    pub fn new(tables: Box<dyn Tables<Database = T>>) -> Self {
        Self { tables }
    }
}

impl DB {
    pub fn new_sqlite(pool: sqlx::Pool<sqlx::Sqlite>) -> Self {
        let tables = SqliteTables::new(pool);
        Self::new(tables)
    }
}

