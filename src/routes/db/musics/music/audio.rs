use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::states::DB;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AudioResponse {
    pub file_id: Uuid,
}

#[get("/audio")]
pub async fn get_audio(db: web::Data<DB>, path: web::Path<String>) -> impl Responder {
    let music_id = path.into_inner();

    if let Ok(music_audio_ref) = db
        .tables
        .refs()
        .music_audio()
        .get_where_music_id(music_id)
        .await
    {
        let audio_opt = db
            .tables
            .audios()
            .get(music_audio_ref.audio.to_string())
            .await;

        if let Ok(audio) = audio_opt {
            let res = AudioResponse {
                file_id: audio.file,
            };
            return HttpResponse::Ok().json(res);
        }
    }

    HttpResponse::NoContent().into()
}
