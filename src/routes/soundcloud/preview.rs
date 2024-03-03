use crate::routes;
use actix_web::{get, web, HttpResponse};
use reqwest::Url;
use sclouddl::track::Track;

pub async fn get_track_with_client_id(
    soundcloud_url: String,
) -> Result<(Track, String), routes::Error> {
    let url = Url::parse(&soundcloud_url).map_err(routes::Error::ParseUrl)?;

    let response = sclouddl::request::ScloudRequest::new(url)
        .send()
        .await
        .map_err(routes::Error::Scloud)?;

    let client_id = response
        .get_client_id()
        .await
        .map_err(routes::Error::Scloud)?;

    let context = response.context().map_err(routes::Error::Scloud)?;

    match context.kind {
        sclouddl::response::ScloudKind::Track(track) => Ok((track, client_id)),
        _ => todo!(),
    }
}

pub async fn get_track(soundcloud_url: String) -> Result<Track, routes::Error> {
    let url = Url::parse(&soundcloud_url).map_err(routes::Error::ParseUrl)?;

    let response = sclouddl::request::ScloudRequest::new(url)
        .send()
        .await
        .map_err(routes::Error::Scloud)?;

    let context = response.context().map_err(routes::Error::Scloud)?;

    match context.kind {
        sclouddl::response::ScloudKind::Track(track) => Ok(track),
        _ => todo!(),
    }
}

#[get("/metadata/{soundcloud_url}")]
pub async fn metadata(soundcloud_url: web::Path<String>) -> Result<HttpResponse, routes::Error> {
    let track = get_track(soundcloud_url.into_inner()).await?;
    Ok(HttpResponse::Ok().json(track))
}
