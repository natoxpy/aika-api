use actix_web::{post, web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::{content::Music, content_refs::{MusicArtistRef, MusicImageRef}},
    states::DB,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportData {
    pub title: String,
    pub image_id: Uuid,
    pub soundcloud_url: String,
    pub artists_id: Vec<Uuid>,
    pub featured_artists_id: Vec<Uuid>,
    pub album_id: Option<Uuid>,
}

#[post("/import")]
async fn import(data: web::Json<ImportData>, db: web::Data<DB>) -> impl Responder {
    let music = Music {
        id: Uuid::new_v4(),
        name: data.title.clone(),
    };

    for artist in data.artists_id.iter() {
        let music_artist = MusicArtistRef {
            id: Uuid::new_v4(),
            music: music.id,
            artist: artist.clone(),
        };

        db.tables.refs().music_artist().save(music_artist);
    }

    for featured_artist in data.featured_artists_id.iter() {
        let music_artist = MusicArtistRef {
            id: Uuid::new_v4(),
            music: music.id,
            artist: featured_artist.clone(),
        };

        db.tables.refs().music_artist().save(music_artist);
    }

    let music_cover = MusicImageRef {
        id: Uuid::new_v4(),
        music: music.id,
        image: data.image_id,
    };

    db.tables.music().save(music.clone());
    db.tables.refs().music_image().save(music_cover);

    HttpResponse::Ok().body(serde_json::to_string(&music).unwrap())
}
