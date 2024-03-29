use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::states::DB;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CoverResponse {
    pub file_id: Uuid,
}

#[get("/cover")]
pub async fn get_cover(db: web::Data<DB>, path: web::Path<String>) -> impl Responder {
    let music_id = path.into_inner();

    if let Some(music_image_ref) = db.tables.refs().music_image().get_where_music_id(music_id).await {
        let image_opt = db.tables.image().get(music_image_ref.image.to_string()).await;

        if let Some(image) = image_opt {
            let res = CoverResponse {
                file_id: image.file,
            };
            return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap());
        }
    }

    HttpResponse::NoContent().into()
}
