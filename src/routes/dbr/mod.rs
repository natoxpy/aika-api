mod musics;
use actix_web::{web, Scope};

pub fn scope() -> Scope {
    web::scope("/db").service(musics::scope())
}
