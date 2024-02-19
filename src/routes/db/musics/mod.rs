pub mod music;

use actix_web::{get, web, Responder, Scope, HttpResponse};

use crate::states::DB;

#[get("/")]
pub async fn get_musics(db: web::Data<DB>) -> impl Responder {
    // for music in db.music_table.all().await {
    //     let image_refs = db.music_image_table.get_by_music(&music).await;
    //     for image_ref in image_refs {
    //         let image = db.image_table.get(image_ref.image).await.unwrap();
    //         let file = db.file_table.get(image.file).await.unwrap();
    //         let artist_refs = db.music_artist_table.get_from_music(&music).await;

    //         for artist_ref in artist_refs {
    //             let artist = db.artist_table.get(artist_ref.artist).await.unwrap();
    //             println!("{:#?}", artist);
    //         }

    //         println!("{:#?}", music);
    //         println!("{:#?}", file);
    //     }
    // }

    HttpResponse::Ok().json(db.tables.music().get_all().await)
}

pub fn scope() -> Scope {
    web::scope("/musics")
        .service(get_musics)
        .service(music::scope())
}
