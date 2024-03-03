// pub mod download;
// pub mod fetch;
use actix_web::{web, Scope};

pub fn scope() -> Scope {
    web::scope("/nyaa")
    //.service(download::download_nyaa)
    //.service(fetch::nyaa_files)
    //.service(fetch::search)
}
