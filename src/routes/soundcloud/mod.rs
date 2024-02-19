pub mod import;
pub mod preview;
pub mod download;

use actix_web::{web, Scope};

pub fn scope() -> Scope {
    web::scope("/soundcloud")
        .service(import::import)
        .service(preview::metadata)
        // .service(download::download_scloud)
}
