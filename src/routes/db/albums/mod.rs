mod album;

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder, Scope};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::content::Album, states::DB};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AlbumData {
    name: String,
    cover: Option<Uuid>,
}

#[get("/")]
pub async fn fetch(db: web::Data<DB>) -> impl Responder {
    HttpResponse::Ok().json(db.tables.albums().get_all().await.unwrap())
}

#[post("/")]
pub async fn create(db: web::Data<DB>, data: web::Json<AlbumData>) -> impl Responder {
    let album_data = data.into_inner();

    let album = Album {
        id: Uuid::new_v4(),
        name: album_data.name,
        cover: None,
        released: None,
    };

    db.tables.albums().save(album.clone()).await.unwrap();

    HttpResponse::Ok().json(album)
}

#[get("/{id}")]
pub async fn read(db: web::Data<DB>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();

    if let Some(album) = db.tables.albums().get(id.to_string()).await {
        return HttpResponse::Ok().json(album);
    }

    HttpResponse::NotFound().into()
}

#[patch("/{id}")]
pub async fn update(
    _db: web::Data<DB>,
    _path: web::Path<Uuid>,
    _data: web::Json<AlbumData>,
) -> impl Responder {
    todo!("implement patch HTTP method to albums");
    #[allow(unreachable_code)]
    ""
}

#[delete("/{id}")]
pub async fn delete(_db: web::Data<DB>, _path: web::Path<Uuid>) -> impl Responder {
    todo!("implement delete HTTP method to albums");
    #[allow(unreachable_code)]
    ""
}

pub fn scope() -> Scope {
    web::scope("/albums")
        .service(fetch)
        .service(create)
        .service(read)
        .service(update)
        .service(delete)
        .service(album::scope())
}
