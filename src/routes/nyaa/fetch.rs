use actix_web::{get, web, Responder};
use nyaadl::{nyaa::{self, Nyaa}, search::SearchParams};
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchQuery {
    query: String,
}

#[get("/search")]
async fn search(q: web::Query<SearchQuery>) -> impl Responder {
    let params = SearchParams::query(&q.query);
    let nyaas = nyaa::get(params).await.unwrap();

    serde_json::to_string(&nyaas)
}

#[get("/files/{id}")]
async fn nyaa_files(spath: web::Path<String>) -> impl Responder {
    let nyaa_id: String = spath.into_inner();
    let files = Nyaa::files_from_id(&nyaa_id).await;

    serde_json::to_string(&files)
}
