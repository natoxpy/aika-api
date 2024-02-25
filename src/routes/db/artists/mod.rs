pub mod artist;

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder, Scope};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::content::Artist, states::DB};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ArtistData {
    name: String,
}

#[get("/")]
pub async fn fetch(db: web::Data<DB>) -> impl Responder {
    HttpResponse::Ok().json(db.tables.artists().get_all().await.unwrap())
}

#[post("/")]
pub async fn create(db: web::Data<DB>, data: web::Json<ArtistData>) -> impl Responder {
    let artist_data = data.into_inner();

    let artist = Artist {
        id: Uuid::new_v4(),
        name: artist_data.name,
        avatar: None,
    };

    db.tables.artists().save(artist.clone()).await.unwrap();

    HttpResponse::Ok().json(artist)
}

#[get("/{id}")]
pub async fn read(db: web::Data<DB>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();

    if let Ok(artist) = db.tables.artists().get(id.to_string()).await {
        return HttpResponse::Ok().json(artist);
    }

    HttpResponse::NotFound().into()
}

#[patch("/{id}")]
pub async fn update(
    _db: web::Data<DB>,
    _path: web::Path<Uuid>,
    _data: web::Json<ArtistData>,
) -> impl Responder {
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
        .service(fetch)
        .service(create)
        .service(read)
        .service(update)
        .service(delete)
        .service(artist::scope())
}
