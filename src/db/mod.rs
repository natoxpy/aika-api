pub mod content;
pub mod sqlite;
pub mod tables;

use paste::paste;
use sqlx::{ColumnIndex, Row};
use std::{future::Future, pin::Pin};

/// Table which implements all generic fetching methods using the ID and generic save methods
pub trait Table<'r, R>
where
    R: Row,
    &'r str: ColumnIndex<R>,
    String: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
    i64: sqlx::decode::Decode<'r, R::Database> + sqlx::types::Type<R::Database>,
{
    type Item: sqlx::FromRow<'r, R>;
    type Database;

    fn get<Q: ToString + Send + 'static>(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Option<Self::Item>> + Send>>;

    fn get_many<Q: ToString + Send + 'static>(
        &self,
        id: Q,
    ) -> Pin<Box<dyn Future<Output = Vec<Self::Item>> + Send>>;

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
