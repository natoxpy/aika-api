use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::states::DB;

#[derive(FromRow, Serialize, Deserialize)]
struct ArtistFeatured {
    pub id: String,
    pub name: String,
    pub avatar: Option<String>,
    pub featured: bool,
}

#[get("/artists")]
pub async fn get_artists(db: web::Data<DB>, path: web::Path<String>) -> impl Responder {
    let music_id = path.into_inner();
    let pool = db.pool.clone();

    let query = "
        select artists.*, music_artists.featured
            from music_artists 
        right join artists 
            on music_artists.artist = artists.id
        where music_artists.music = $1";

    let artists = sqlx::query_as::<_, ArtistFeatured>(query)
        .bind(music_id.to_string())
        .fetch_all(&pool)
        .await
        .map_err(crate::db::Error::Sqlx)
        .unwrap();

    HttpResponse::Ok().json(artists)
}
