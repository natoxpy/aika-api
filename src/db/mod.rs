pub mod content;
pub mod content_refs;
pub mod sqlite;

use paste::paste;
use std::{future::Future, pin::Pin};

use self::{
    content::{Album, Artist, Audio, File, Image, Music},
    content_refs::{AlbumArtistRef, MusicAlbumRef, MusicArtistRef, MusicAudioRef, MusicImageRef},
};

pub trait RefTables {
    type Database: sqlx::Database;

    fn album_artist(
        &self,
    ) -> Box<
        &dyn TableAlbumArtistRef<
            Item = AlbumArtistRef,
            ItemWhereAlbum = AlbumArtistRef,
            ItemWhereArtist = AlbumArtistRef,
            Database = Self::Database,
        >,
    >;

    fn music_album(
        &self,
    ) -> Box<
        &dyn TableMusicAlbumRef<
            Item = MusicAlbumRef,
            ItemWhereMusic = MusicAlbumRef,
            ItemWhereAlbum = MusicAlbumRef,
            Database = Self::Database,
        >,
    >;

    fn music_artist(
        &self,
    ) -> Box<
        &dyn TableMusicArtistRef<
            Item = MusicArtistRef,
            ItemWhereMusic = MusicArtistRef,
            ItemWhereArtist = MusicArtistRef,
            Database = Self::Database,
        >,
    >;

    fn music_image(
        &self,
    ) -> Box<
        &dyn TableMusicImageRef<
            Item = MusicImageRef,
            ItemWhereMusic = MusicImageRef,
            ItemWhereImage = MusicImageRef,
            Database = Self::Database,
        >,
    >;

    fn music_audio(
        &self,
    ) -> Box<
        &dyn TableMusicAudioRef<
            Item = MusicAudioRef,
            ItemWhereMusic = MusicAudioRef,
            ItemWhereAudio = MusicAudioRef,
            Database = Self::Database,
        >,
    >;
}

pub trait Tables {
    type Database: sqlx::Database;

    fn new(pool: sqlx::Pool<Self::Database>) -> Box<dyn Tables<Database = Self::Database>>
    where
        Self: Sized;

    fn musics(&self) -> Box<&dyn Table<Item = Music, Database = Self::Database>>;
    fn albums(&self) -> Box<&dyn Table<Item = Album, Database = Self::Database>>;
    fn images(&self) -> Box<&dyn Table<Item = Image, Database = Self::Database>>;
    fn artists(&self) -> Box<&dyn Table<Item = Artist, Database = Self::Database>>;
    fn audios(&self) -> Box<&dyn Table<Item = Audio, Database = Self::Database>>;
    fn files(&self) -> Box<&dyn Table<Item = File, Database = Self::Database>>;

    fn refs(&self) -> Box<&dyn RefTables<Database = Self::Database>>;
}

/// Table which implements all generic fetching methods using the ID and generic save methods
pub trait Table<Q: ToString + Send + 'static = String> {
    type Item;
    type Database: sqlx::Database;

    fn get(&self, id: Q) -> Pin<Box<dyn Future<Output = Option<Self::Item>> + Send>>;

    fn get_many(&self, id: Q) -> Pin<Box<dyn Future<Output = Vec<Self::Item>> + Send>>;

    fn get_all(&self) -> Pin<Box<dyn Future<Output = Vec<Self::Item>> + Send>>;

    fn save(&self, item: Self::Item) -> Pin<Box<dyn Future<Output = ()> + Send>>;

    fn save_many(&self, items: Vec<Self::Item>) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

macro_rules! TableSearchFor {
    ($a:ident) => {
        paste! {
            pub trait [<TableFetchWhere $a>]<Q: ToString + Send + 'static = String> {
                type [<Item Where $a>];
                fn [<get_where_ $a:lower>](&self, [<$a:lower>]: $a) -> Pin<Box<dyn Future<Output = Option<Self::[<Item Where $a>]>> + Send>>;
                fn [<get_where_ $a:lower _id>](&self, id: Q) -> Pin<Box<dyn Future<Output = Option<Self::[<Item Where $a>]>> + Send>>;

            }
        }
    };
}

TableSearchFor!(Music);
TableSearchFor!(Image);
TableSearchFor!(Audio);
TableSearchFor!(Artist);
TableSearchFor!(Album);

pub trait TableMusicArtistRef: Table + TableFetchWhereMusic + TableFetchWhereArtist {}
pub trait TableMusicImageRef: Table + TableFetchWhereMusic + TableFetchWhereImage {}
pub trait TableMusicAudioRef: Table + TableFetchWhereMusic + TableFetchWhereAudio {}
pub trait TableMusicAlbumRef: Table + TableFetchWhereMusic + TableFetchWhereAlbum {}
pub trait TableAlbumArtistRef: Table + TableFetchWhereArtist + TableFetchWhereAlbum {}
