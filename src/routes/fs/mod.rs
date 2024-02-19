pub mod upload;

use actix_web::{web, Scope};

pub fn scope() -> Scope {
    web::scope("/fs").service(upload::upload_from_url)
}
