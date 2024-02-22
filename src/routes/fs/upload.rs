use crate::db::content::{self, File};
use actix_web::{post, web, HttpResponse, Responder};
use std::{fs, io::Write, path::PathBuf};
use uuid::Uuid;

use crate::states::{DB, FILES};

pub async fn upload_from_url_standalone(file_url: String) -> File {
    let response = reqwest::get(file_url).await.unwrap();
    let body = response.bytes().await.unwrap();
    let mime = infer::get(&body).unwrap();
    let id = Uuid::new_v4();

    let location = "/";
    let file_name = format!("{}.{}", id.to_string(), mime.extension());

    let path = PathBuf::from(format!("{}{}{}", FILES, location, file_name));
    let mut file_fs = fs::File::create(path).unwrap();

    file_fs.write_all(&body).unwrap();

    content::File {
        id,
        name: file_name.to_string(),
        location: format!("{}{}", location, file_name),
        mime: mime.to_string(),
        size: body.len() as u64,
    }
}

#[post("/upload/{file_url}")]
async fn upload_from_url(db: web::Data<DB>, file_url: web::Path<String>) -> impl Responder {
    let file_record = upload_from_url_standalone(file_url.into_inner()).await;

    db.tables.files().save(file_record.clone()).await;

    HttpResponse::Ok().json(file_record)
}
