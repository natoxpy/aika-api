extern crate music_manager;

use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use music_manager::{
    db::tables::{
        artist::ArtistTable,
        cross_ref::{
            music_artists::MusicArtistTable, music_audios::MusicAudioTable,
            music_images::MusicImageTable,
        },
        files::FileTable,
        images::ImageTable,
        musics::MusicTable, audios::AudioTable,
    },
    routes::{cdn, dbr, nyaa, soundcloud, youtube},
    states::{DBState, FILES},
};
use sqlx::sqlite::SqlitePoolOptions;
use std::env;

#[get("/")]
async fn hello_world(db: web::Data<DBState>) -> impl Responder {
    let table = &db.music_table;
    // table.save(Music::new("from a world of love")).await;
    table.all().await;

    HttpResponse::Ok().body("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut sqlpath = std::path::PathBuf::new();
    env::set_var("RUST_BACKTRACE", "1");

    sqlpath.push(FILES);
    sqlpath.push("music.db");

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(sqlpath.to_str().unwrap())
        .await
        .unwrap();

    let tables = web::Data::new(DBState {
        music_table: MusicTable::new(pool.clone()).await,
        file_table: FileTable::new(pool.clone()).await,
        image_table: ImageTable::new(pool.clone()).await,
        audio_table: AudioTable::new(pool.clone()).await,
        artist_table: ArtistTable::new(pool.clone()).await,
        music_audio_table: MusicAudioTable::new(pool.clone()).await,
        music_image_table: MusicImageTable::new(pool.clone()).await,
        music_artist_table: MusicArtistTable::new(pool.clone()).await,
    });


    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(tables.clone())
            .service(dbr::scope())
            .service(cdn::scope())
            .service(soundcloud::scope())
            .service(nyaa::scope())
            .service(youtube::scope())
            .service(hello_world)
    })
    .bind(("::1", 8000))?
    .run()
    .await
}
