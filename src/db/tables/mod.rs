use uuid::Uuid;

use self::musics::Music;

pub mod audios;
pub mod images;
pub mod musics;
pub mod files;
pub mod cross_ref;
pub mod artist;

/// Allows to extract primary value 
pub trait QueryPrimaryKey {
    fn get_primary_key(&self) -> String;
}

impl QueryPrimaryKey for Music {
    fn get_primary_key(&self) -> String {
        self.id.to_string()
    }
}


impl QueryPrimaryKey for Uuid {
    fn get_primary_key(&self) -> String {
        self.to_string()
    }
}
