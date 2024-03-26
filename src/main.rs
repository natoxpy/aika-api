extern crate music_manager;

use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use music_manager::{
    routes,
    // routes::{cdn, dbr, nyaa, soundcloud, youtube},
    states::{DB, FILES},
};
use sqlx::sqlite::SqlitePoolOptions;
use std::env;

#[get("/")]
async fn hello_world(db: web::Data<DB>) -> impl Responder {
    let table = &db.tables.musics();

    table.get_all().await.unwrap();
    HttpResponse::Ok().body("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut sqlpath = std::path::PathBuf::new();
    env::set_var("RUST_BACKTRACE", "1");

    sqlpath.push(FILES);
    sqlpath.push("core.db");

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(sqlpath.to_str().unwrap())
        .await
        .unwrap();

    HttpServer::new(move || {
        let cors = Cors::permissive();
        let p = pool.clone();
        let db = web::Data::new(DB::new_sqlite(p));

        App::new()
            .wrap(cors)
            .app_data(db.clone())
            .service(routes::db::scope())
            .service(routes::cdn::scope())
            .service(routes::fs::scope())
            .service(routes::soundcloud::scope())
            .service(routes::nyaa::scope())
            .service(routes::youtube::scope())
            .service(hello_world)
    })
    .bind(("::1", 8000))?
    .run()
    .await
}
