pub mod content;
pub mod content_refs;
pub mod error;
pub mod sqlite;

use paste::paste;
use std::{future::Future, pin::Pin};

use self::{
    content::{Album, Artist, Audio, File, Image, Music},
    content_refs::{AlbumArtistRef, MusicAlbumRef, MusicArtistRef, MusicAudioRef, MusicImageRef},
};
pub use error::Error;

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

pub type TableGetReturn<Item> =
    Pin<Box<dyn Future<Output = Result<Item, crate::db::Error>> + Send>>;

pub type TableSaveReturn = Pin<Box<dyn Future<Output = Result<(), crate::db::Error>> + Send>>;

/// Table which implements all generic fetching methods using the ID and generic save methods
pub trait Table<Q: ToString + Send + 'static = String> {
    type Item;
    type Database: sqlx::Database;

    fn get(&self, id: Q) -> TableGetReturn<Self::Item>;
    fn get_many(&self, id: Q) -> TableGetReturn<Vec<Self::Item>>;
    fn get_all(&self) -> TableGetReturn<Vec<Self::Item>>;

    fn save(&self, item: Self::Item) -> TableSaveReturn;
    fn save_many(&self, items: Vec<Self::Item>) -> TableSaveReturn;
}

macro_rules! TableSearchFor {
    ($a:ident) => {
        paste! {
            pub trait [<TableFetchWhere $a>]<Q: ToString + Send + 'static = String> {
                type [<Item Where $a>];
                fn [<get_where_ $a:lower>](&self, [<$a:lower>]: $a) -> Pin<Box<dyn Future<Output = Result<Self::[<Item Where $a>], crate::db::Error>> + Send>>;

                #[allow(unused_variables)]
                fn [<get_all_where_ $a:lower>](&self, [<$a:lower>]: $a) -> Pin<Box<dyn Future<Output = Result<Vec<Self::[<Item Where $a>]>, crate::db::Error>> + Send>> { todo!() }

                fn [<get_where_ $a:lower _id>](&self, id: Q) -> Pin<Box<dyn Future<Output = Result<Self::[<Item Where $a>], crate::db::Error>> + Send>>;

                #[allow(unused_variables)]
                fn [<get_all_where_ $a:lower _id>](&self, id: Q) -> Pin<Box<dyn Future<Output = Result<Vec<Self::[<Item Where $a>]>, crate::db::Error>> + Send>> { todo!() }
            }
        }
    };
}

macro_rules! TableSearchForJoin {
    ($a:ident, $b:ident) => {
        paste! {
            pub trait [<TableFetchWhereJoin $a>]<Q: ToString + Send + 'static = String> {
                type [<Item Where $a>];


                #[allow(unused_variables)]
                fn [<get_where_ $a:lower>, and_or_, $b:lower](&self, [<$a:lower>]: $a, [<$b:lower>: $b]) -> Pin<Box<dyn Future<Output = Result<Self::[<Item Where $a>], crate::db::Error>> + Send>> { todo!() }

                #[allow(unused_variables)]
                fn [<get_where_ $a:lower>, and, $b:lower](&self, [<$a:lower>]: $a, [<$b:lower>: $b]) -> Pin<Box<dyn Future<Output = Result<Self::[<Item Where $a>], crate::db::Error>> + Send>> { todo!() }
            }
        }
    };
}

macro_rules! TableSearchForAllJoin {
    ($a:ident, $b:ident) => {
        paste! {
            pub trait [<TableFetchWhereJoin $a>]<Q: ToString + Send + 'static = String> {
                type [<Item Where $a>];


                #[allow(unused_variables)]
                fn [<get_all_where_ $a:lower>, and_or_, $b:lower](&self, [<$a:lower>]: $a, [<$b:lower>: $b]) -> Pin<Box<dyn Future<Output = Result<Self::[<Item Where $a>], crate::db::Error>> + Send>> { todo!() }

                #[allow(unused_variables)]
                fn [<get_all_where_ $a:lower>, and, $b:lower](&self, [<$a:lower>]: $a, [<$b:lower>: $b]) -> Pin<Box<dyn Future<Output = Result<Self::[<Item Where $a>], crate::db::Error>> + Send>> { todo!() }

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
