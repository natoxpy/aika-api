use actix_web::{get, web, HttpResponse, Responder, Scope};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::states::DB;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AvatarResponse {
    pub file_id: Uuid,
}

#[get("/avatar")]
pub async fn get_avatar(db: web::Data<DB>, path: web::Path<Uuid>) -> impl Responder {
    let artist_id = path.into_inner();

    let artist_opt = db.tables.artists().get(artist_id.to_string()).await;

    if let Some(artist) = artist_opt {
        if let Some(avatar_id) = artist.avatar {
            if let Some(avatar) = db.tables.images().get(avatar_id).await {
                return HttpResponse::Ok().json(AvatarResponse {
                    file_id: avatar.file,
                });
            }
        }
    }

    HttpResponse::NoContent().into()
}

pub fn scope() -> Scope {
    web::scope("/{id}").service(get_avatar)
}
