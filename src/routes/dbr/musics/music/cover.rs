use actix_web::{get, web, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::states::DBState;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CoverResponse {
    pub file_id: Uuid
}

#[get("/cover")]
pub async fn get_cover(db: web::Data<DBState>, path: web::Path<String>) -> impl Responder {
    let music_id = path.into_inner();

    for music_image_ref in db.music_image_table.get_from_music_id(music_id).await {
        let image_opt = db.image_table.get(music_image_ref.image).await;

        if let Some(image) = image_opt {
            let res = CoverResponse { file_id: image.file };
            return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap());
        }
    }

    HttpResponse::NoContent().into()
}
