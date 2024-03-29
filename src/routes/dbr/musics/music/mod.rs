use actix_web::{get, web, HttpResponse, Responder, Scope};

use crate::states::DB;

pub mod audio;
pub mod cover;

#[get("/")]
pub async fn get_music(db: web::Data<DB>, path: web::Path<String>) -> impl Responder {
    let music_id = path.into_inner();

    if let Some(music) = db.tables.music().get(music_id).await {
        return HttpResponse::Ok().body(serde_json::to_string(&music).unwrap());
    } else {
        HttpResponse::NoContent().into()
    }
}

pub fn scope() -> Scope {
    web::scope("/{music_id}")
        .service(cover::get_cover)
        .service(audio::get_audio)
        .service(get_music)
}
