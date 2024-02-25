use actix_web::{delete, get, patch, post, web, HttpResponse, Responder, Scope};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::content::Image, states::DB};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ImageData {
    file: Uuid,
}

#[get("/")]
pub async fn fetch(db: web::Data<DB>) -> impl Responder {
    HttpResponse::Ok().json(db.tables.images().get_all().await.unwrap())
}

#[post("/")]
pub async fn create(db: web::Data<DB>, data: web::Json<ImageData>) -> impl Responder {
    let image_data = data.into_inner();

    let image = Image {
        id: Uuid::new_v4(),
        file: image_data.file,
    };

    db.tables.images().save(image.clone()).await.unwrap();

    HttpResponse::Ok().json(image)
}

#[get("/{id}")]
pub async fn read(db: web::Data<DB>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();

    if let Ok(image) = db.tables.images().get(id.to_string()).await {
        return HttpResponse::Ok().json(image);
    }

    HttpResponse::NotFound().into()
}

#[patch("/{id}")]
pub async fn update(
    _db: web::Data<DB>,
    _path: web::Path<Uuid>,
    _data: web::Json<ImageData>,
) -> impl Responder {
    todo!("implement patch HTTP method to images");
    #[allow(unreachable_code)]
    ""
}

#[delete("/{id}")]
pub async fn delete(_db: web::Data<DB>, _path: web::Path<Uuid>) -> impl Responder {
    todo!("implement delete HTTP method to images");
    #[allow(unreachable_code)]
    ""
}

pub fn scope() -> Scope {
    web::scope("/images")
        .service(fetch)
        .service(create)
        .service(read)
        .service(update)
        .service(delete)
}
