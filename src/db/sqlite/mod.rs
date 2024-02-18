use self::{
    artists::ArtistTable,
    audios::AudioTable,
    files::FileTable,
    images::ImageTable,
    musics::MusicTable,
    refs::{
        music_artists::MusicArtistTable, music_audios::MusicAudioTable,
        music_images::MusicImageTable,
    },
};
use super::{RefTables, Tables};

pub mod albums;
pub mod artists;
pub mod audios;
pub mod files;
pub mod images;
pub mod musics;
pub mod refs;

pub struct SqliteRefTables {
    pub music_artist_table: MusicArtistTable,
    pub music_audio_table: MusicAudioTable,
    pub music_image_table: MusicImageTable,
}

pub struct SqliteTables {
    pub pool: sqlx::Pool<sqlx::Sqlite>,
    pub music_table: MusicTable,
    pub image_table: ImageTable,
    pub artist_table: ArtistTable,
    pub audio_table: AudioTable,
    pub file_table: FileTable,
    pub ref_tables: SqliteRefTables,
}

impl Tables for SqliteTables {
    type Database = sqlx::Sqlite;

    fn refs(&self) -> Box<&dyn super::RefTables<Database = Self::Database>> {
        Box::new(&self.ref_tables)
    }

    fn new(pool: sqlx::Pool<Self::Database>) -> Box<dyn Tables<Database = sqlx::Sqlite>>
    where
        Self: Sized,
    {
        let ref_tables = SqliteRefTables {
            music_artist_table: MusicArtistTable { pool: pool.clone() },
            music_image_table: MusicImageTable { pool: pool.clone() },
            music_audio_table: MusicAudioTable { pool: pool.clone() },
        };

        Box::new(Self {
            pool: pool.clone(),
            music_table: MusicTable { pool: pool.clone() },
            image_table: ImageTable { pool: pool.clone() },
            artist_table: ArtistTable { pool: pool.clone() },
            audio_table: AudioTable { pool: pool.clone() },
            file_table: FileTable { pool: pool.clone() },
            ref_tables,
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

impl RefTables for SqliteRefTables {
    type Database = sqlx::Sqlite;

    fn music_artist(
        &self,
    ) -> Box<
        &dyn super::TableMusicArtistRef<
            Item = super::content_refs::MusicArtistRef,
            ItemWhereMusic = super::content_refs::MusicArtistRef,
            ItemWhereArtist = super::content_refs::MusicArtistRef,
            Database = Self::Database,
        >,
    > {
        Box::new(&self.music_artist_table)
    }

    fn music_image(
        &self,
    ) -> Box<
        &dyn super::TableMusicImageRef<
            Item = super::content_refs::MusicImageRef,
            ItemWhereMusic = super::content_refs::MusicImageRef,
            ItemWhereImage = super::content_refs::MusicImageRef,
            Database = Self::Database,
        >,
    > {
        Box::new(&self.music_image_table)
    }

    fn music_audio(
        &self,
    ) -> Box<
        &dyn super::TableMusicAudioRef<
            Item = super::content_refs::MusicAudioRef,
            ItemWhereMusic = super::content_refs::MusicAudioRef,
            ItemWhereAudio = super::content_refs::MusicAudioRef,
            Database = Self::Database,
        >,
    > {
        Box::new(&self.music_audio_table)
    }
}
