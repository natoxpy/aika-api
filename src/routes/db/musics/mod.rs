pub mod music;

use actix_web::{get, web, HttpResponse, Responder, Scope};

use crate::states::DB;

#[get("/")]
pub async fn get_musics(db: web::Data<DB>) -> impl Responder {
    HttpResponse::Ok().json(db.tables.musics().get_all().await.unwrap())
}

pub fn scope() -> Scope {
    web::scope("/musics")
        .service(get_musics)
        .service(music::scope())
}
