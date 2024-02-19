use actix_web::{delete, get, patch, post, web, HttpResponse, Responder, Scope};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::content::Image, states::DB};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ImageData {
    file: Uuid,
}

#[post("/")]
pub async fn create(db: web::Data<DB>, data: web::Json<ImageData>) -> impl Responder {
    let image_data = data.into_inner();

    let image = Image {
        id: Uuid::new_v4(),
        file: image_data.file,
    };

    db.tables.image().save(image.clone()).await;

    HttpResponse::Ok().body(serde_json::to_string(&image).unwrap())
}

#[get("/{id}")]
pub async fn read(db: web::Data<DB>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();

    if let Some(image) = db.tables.image().get(id.to_string()).await {
        return HttpResponse::Ok().body(serde_json::to_string(&image).unwrap())
    }

    HttpResponse::NotFound().into()
}

#[patch("/{id}")]
pub async fn update(_db: web::Data<DB>, _path: web::Path<Uuid>, _data: web::Json<ImageData>) -> impl Responder {
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
        .service(create)
        .service(read)
        .service(update)
        .service(delete)
}
