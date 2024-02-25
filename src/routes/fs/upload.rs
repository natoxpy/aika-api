use crate::db::content::{self, File};
use actix_web::{post, web, HttpResponse, Responder};
use std::{fs, io::Write, path::PathBuf};
use uuid::Uuid;

use crate::states::{DB, FILES};

pub async fn get_bytes_from_url(file_url: String) -> Vec<u8> {
    let response = reqwest::get(file_url).await.unwrap();
    response.bytes().await.unwrap().to_vec()
}

pub async fn upload_from_bytes(bytes: Vec<u8>) -> File {
    let mime = infer::get(&bytes).unwrap();
    let id = Uuid::new_v4();

    let location = "/";
    let file_name = format!("{}.{}", id.to_string(), mime.extension());

    let path = PathBuf::from(format!("{}{}{}", FILES, location, file_name));
    let mut file_fs = fs::File::create(path).unwrap();

    file_fs.write_all(&bytes).unwrap();

    content::File {
        id,
        name: file_name.to_string(),
        location: format!("{}{}", location, file_name),
        mime: mime.to_string(),
        size: bytes.len() as u64,
    }
}

#[post("/upload")]
async fn upload_from_buffer(db: web::Data<DB>, data: web::Bytes) -> impl Responder {
    let file = upload_from_bytes(data.to_vec()).await;
    db.tables.files().save(file.clone()).await.unwrap();
    HttpResponse::Ok().json(&file)
}

#[post("/upload/{file_url}")]
async fn upload_from_url(db: web::Data<DB>, file_url: web::Path<String>) -> impl Responder {
    let file_record = upload_from_bytes(get_bytes_from_url(file_url.into_inner()).await).await;

    db.tables.files().save(file_record.clone()).await.unwrap();

    HttpResponse::Ok().json(file_record)
}
