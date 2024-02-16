use actix_web::{get, web, Responder};
use nyaadl::nyaa::Nyaa;

#[get("/download/{id}")]
async fn download_nyaa(spath: web::Path<String>) -> impl Responder {
    let nyaa_id: String = spath.into_inner();
    let nyaa = Nyaa::from_id(&nyaa_id).await.unwrap();

    serde_json::to_string(&nyaa)
}

