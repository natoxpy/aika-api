use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::{
        content::{Audio, Music},
        content_refs::{MusicAlbumRef, MusicArtistRef, MusicAudioRef, MusicImageRef},
    },
    routes::{
        self,
        fs::upload::{get_bytes_from_url, upload_from_bytes},
        soundcloud::preview::get_track_with_client_id,
    },
    states::DB,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundcloudImport {
    pub title: String,
    pub soundcloud_url: String,
    pub artists_id: Vec<Uuid>,
    pub featured_artists_id: Vec<Uuid>,
    pub albums_id: Vec<Uuid>,
    pub image_id: Uuid,
}

#[post("/import")]
async fn import(
    data: web::Json<SoundcloudImport>,
    db: web::Data<DB>,
) -> Result<HttpResponse, routes::Error> {
    let music = Music {
        id: Uuid::new_v4(),
        name: data.title.clone(),
    };

    db.tables
        .musics()
        .save(music.clone())
        .await
        .map_err(|err| routes::Error::DB(err))?;

    let (track, client_id) = get_track_with_client_id(data.soundcloud_url.clone()).await?;

    let progressive_index = track
        .media
        .iter()
        .position(|item| item.get_format_protocol() == "progressive")
        .ok_or(routes::Error::Other(String::from(
            "Progressive media not found",
        )))?;

    let media = track
        .media
        .get(progressive_index)
        .ok_or(routes::Error::Other(String::from("no media url")))?
        .get_url(client_id)
        .await
        .map_err(|err| routes::Error::Scloud(err))?;

    let uploaded_file = upload_from_bytes(get_bytes_from_url(media).await?).await?;

    let audio_record = Audio {
        id: Uuid::new_v4(),
        file: uploaded_file.id,
    };

    let music_audio_ref = MusicAudioRef {
        id: Uuid::new_v4(),
        music: music.id,
        audio: audio_record.id,
    };

    db.tables
        .files()
        .save(uploaded_file)
        .await
        .map_err(|err| routes::Error::DB(err))?;

    db.tables
        .audios()
        .save(audio_record)
        .await
        .map_err(|err| routes::Error::DB(err))?;

    db.tables
        .refs()
        .music_audio()
        .save(music_audio_ref)
        .await
        .map_err(|err| routes::Error::DB(err))?;

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
            .map_err(|err| routes::Error::DB(err))?;
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
            .map_err(|err| routes::Error::DB(err))?;
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
            .map_err(|err| routes::Error::DB(err))?;
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
        .map_err(|err| routes::Error::DB(err))?;

    Ok(HttpResponse::Ok().json(music))
}
