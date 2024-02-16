use crate::db::tables::{
    cross_ref::{music_audios::MusicAudioTable, music_artists::MusicArtistTable}, cross_ref::music_images::MusicImageTable,
    files::FileTable, images::ImageTable, musics::MusicTable, artist::ArtistTable, audios::AudioTable,
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
