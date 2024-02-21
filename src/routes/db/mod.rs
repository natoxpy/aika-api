mod albums;
mod artists;
mod audios;
mod images;
mod musics;

use actix_web::{web, Scope};
pub fn scope() -> Scope {
    web::scope("/db")
        .service(musics::scope())
        .service(images::scope())
        .service(artists::scope())
        .service(audios::scope())
        .service(albums::scope())
}
