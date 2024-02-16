pub mod file;
pub mod get_file;

use actix_web::{web, Scope};

pub fn scope() -> Scope {
    web::scope("/cdn")
        // .service(get_file::get_file)
        .service(file::file_cdn)
}
