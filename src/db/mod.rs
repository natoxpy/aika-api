pub mod content;
pub mod sqlite;
pub mod tables;

use paste::paste;
use std::{future::Future, pin::Pin};

use self::content::{Artist, Audio, File, Image, Music};

pub trait Tables {
    type Database: sqlx::Database;

    fn new(pool: sqlx::Pool<Self::Database>) -> Box<dyn Tables<Database = Self::Database>>
    where
        Self: Sized;

    fn music(&self) -> Box<&dyn Table<Item = Music, Database = Self::Database>>;
    fn image(&self) -> Box<&dyn Table<Item = Image, Database = Self::Database>>;
    fn artists(&self) -> Box<&dyn Table<Item = Artist, Database = Self::Database>>;
    fn audio(&self) -> Box<&dyn Table<Item = Audio, Database = Self::Database>>;
    fn file(&self) -> Box<&dyn Table<Item = File, Database = Self::Database>>;
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
            pub trait [<TableFetchWhere $a>]<T> {
                fn [<get_where_ $a:lower>](&self) -> Pin<Box<dyn Future<Output = T> + Send>>;
                fn [<get_where_ $a:lower _id>]<Q: ToString + Send + 'static>(&self) -> Pin<Box<dyn Future<Output = T> + Send>>;
            }
        }
    };
}

TableSearchFor!(Music);
TableSearchFor!(Image);
TableSearchFor!(Audio);
TableSearchFor!(File);
