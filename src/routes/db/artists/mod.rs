use actix_web::{delete, get, patch, post, web, HttpResponse, Responder, Scope};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::content::Artist, states::DB};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ArtistData {
    name: String,
}

#[post("/")]
pub async fn create(db: web::Data<DB>, data: web::Json<ArtistData>) -> impl Responder {
    let artist_data = data.into_inner();

    let artist = Artist {
        id: Uuid::new_v4(),
        name: artist_data.name,
    };

    db.tables.artists().save(artist.clone()).await;

    HttpResponse::Ok().body(serde_json::to_string(&artist).unwrap())
}

#[get("/{id}")]
pub async fn read(db: web::Data<DB>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();

    if let Some(artist) = db.tables.artists().get(id.to_string()).await {
        return HttpResponse::Ok().body(serde_json::to_string(&artist).unwrap())
    }

    HttpResponse::NotFound().into()
}

#[patch("/{id}")]
pub async fn update(_db: web::Data<DB>, _path: web::Path<Uuid>, _data: web::Json<ArtistData>) -> impl Responder {
    todo!("implement patch HTTP method to artists");
    #[allow(unreachable_code)]
    ""
}

#[delete("/{id}")]
pub async fn delete(_db: web::Data<DB>, _path: web::Path<Uuid>) -> impl Responder {
    todo!("implement delete HTTP method to artists");
    #[allow(unreachable_code)]
    ""
}

pub fn scope() -> Scope {
    web::scope("/artists")
        .service(create)
        .service(read)
        .service(update)
        .service(delete)
}
