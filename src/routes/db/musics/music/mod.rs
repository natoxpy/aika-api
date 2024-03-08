use actix_web::{get, web, HttpResponse, Responder, Scope};

use crate::states::DB;

pub mod artists;
pub mod audio;
pub mod cover;

#[get("/")]
pub async fn get_music(db: web::Data<DB>, path: web::Path<String>) -> impl Responder {
    let music_id = path.into_inner();

    if let Ok(music) = db.tables.musics().get(music_id).await {
        HttpResponse::Ok().json(music)
    } else {
        HttpResponse::NoContent().into()
    }
}

pub fn scope() -> Scope {
    web::scope("/{music_id}")
        .service(cover::get_cover)
        .service(audio::get_audio)
        .service(artists::get_artists)
        .service(get_music)
}
