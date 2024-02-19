use actix_web::{delete, get, patch, post, web, HttpResponse, Responder, Scope};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::content::Audio, states::DB};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AudioData {
    file: Uuid,
}

#[post("/")]
pub async fn create(db: web::Data<DB>, data: web::Json<AudioData>) -> impl Responder {
    let audio_data = data.into_inner();

    let audio = Audio {
        id: Uuid::new_v4(),
        file: audio_data.file,
    };

    db.tables.audio().save(audio.clone()).await;

    HttpResponse::Ok().json(audio)
}

#[get("/{id}")]
pub async fn read(db: web::Data<DB>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();

    if let Some(audio) = db.tables.audio().get(id.to_string()).await {
        return HttpResponse::Ok().json(audio)
    }

    HttpResponse::NotFound().into()
}

#[patch("/{id}")]
pub async fn update(_db: web::Data<DB>, _path: web::Path<Uuid>, _data: web::Json<AudioData>) -> impl Responder {
    todo!("implement patch HTTP method to audios");
    #[allow(unreachable_code)]
    ""
}

#[delete("/{id}")]
pub async fn delete(_db: web::Data<DB>, _path: web::Path<Uuid>) -> impl Responder {
    todo!("implement delete HTTP method to audios");
    #[allow(unreachable_code)]
    ""
}

pub fn scope() -> Scope {
    web::scope("/audios")
        .service(create)
        .service(read)
        .service(update)
        .service(delete)
}
