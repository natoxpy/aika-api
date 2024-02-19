use actix_web::{get, web, Responder, HttpResponse};
use sclouddl::{track::Track, utils::gen_key};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StreamTrack {
    progressive: String,
    track: Track
}

#[get("/download/{scloud_url}")]
async fn download_scloud(spath: web::Path<String>) -> impl Responder {
    let scloud_url: String = spath.into_inner();
    let track = Track::get_song(&scloud_url).await;

    let key = gen_key().await.unwrap();
    let urls = track.media.get_urls(&key).await;
    println!("{:#?}", urls);

    ""
}

#[get("/progressive/{scloud_url}")]
async fn get_progressive_stream(spath: web::Path<String>) -> impl Responder {
    let _scloud_url: String = spath.into_inner();

    let url = "https://soundcloud.com/braeden-richey/mili-in-hell-we-live-lament-lets-lament";
    let track = Track::get_song(&url).await;

    let key = gen_key().await.unwrap();
    let urls = track.media.get_progressive(&key).await;

    let res = StreamTrack {
        progressive: urls,
        track
    };

    HttpResponse::Ok().json(res)
}
