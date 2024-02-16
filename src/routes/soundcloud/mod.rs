pub mod download;
use actix_web::{web, Scope};

pub fn scope() -> Scope {
    web::scope("/soundcloud")
        .service(download::download_scloud)
        .service(download::get_progressive_stream)
        // .service(download::download_scloud)
}
