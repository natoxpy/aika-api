use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::{
        content::Music,
        content_refs::{MusicAlbumRef, MusicArtistRef, MusicImageRef},
    },
    // routes::fs::upload::{get_bytes_from_url, upload_from_bytes},
    states::DB,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportData {
    pub title: String,
    pub image_id: Uuid,
    pub soundcloud_url: String,
    pub artists_id: Vec<Uuid>,
    pub featured_artists_id: Vec<Uuid>,
    pub albums_id: Vec<Uuid>,
}

#[post("/import")]
async fn import(data: web::Json<ImportData>, db: web::Data<DB>) -> impl Responder {
    let music = Music {
        id: Uuid::new_v4(),
        name: data.title.clone(),
    };

    db.tables.musics().save(music.clone()).await.unwrap();

    // let track = Track::get_song(&data.soundcloud_url).await;

    // let media = track.media.get_progressive(&track.client_id).await;
    // let audio_file = upload_from_bytes(get_bytes_from_url(media).await).await;

    // let audio_record = Audio {
    //     id: Uuid::new_v4(),
    //     file: audio_file.id,
    // };

    // let music_audio_ref = MusicAudioRef {
    //     id: Uuid::new_v4(),
    //     music: music.id,
    //     audio: audio_record.id,
    // };

    // db.tables.files().save(audio_file).await;
    // db.tables.audios().save(audio_record).await;

    // db.tables.refs().music_audio().save(music_audio_ref).await;

    for artist in data.artists_id.iter() {
        let music_artist = MusicArtistRef {
            id: Uuid::new_v4(),
            music: music.id,
            artist: artist.clone(),
        };

        db.tables
            .refs()
            .music_artist()
            .save(music_artist)
            .await
            .unwrap();
    }

    for featured_artist in data.featured_artists_id.iter() {
        let music_artist = MusicArtistRef {
            id: Uuid::new_v4(),
            music: music.id,
            artist: featured_artist.clone(),
        };

        db.tables
            .refs()
            .music_artist()
            .save(music_artist)
            .await
            .unwrap();
    }

    for album in data.albums_id.iter() {
        let music_album = MusicAlbumRef {
            id: Uuid::new_v4(),
            music: music.id,
            album: album.clone(),
        };

        db.tables
            .refs()
            .music_album()
            .save(music_album)
            .await
            .unwrap();
    }

    let music_cover = MusicImageRef {
        id: Uuid::new_v4(),
        music: music.id,
        image: data.image_id,
    };

    db.tables
        .refs()
        .music_image()
        .save(music_cover)
        .await
        .unwrap();

    HttpResponse::Ok().json(music)
}
