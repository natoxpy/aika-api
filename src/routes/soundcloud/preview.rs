use actix_web::{get, web, Responder, HttpResponse};
use sclouddl::track::Track;

#[get("/metadata/{soundcloud_url}")]
async fn metadata(soundcloud_url: web::Path<String>) -> impl Responder {
    let track = Track::get_song(&soundcloud_url.into_inner()).await;
    HttpResponse::Ok().json(track)
}
