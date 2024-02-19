use crate::db::content;
use actix_web::{get, web, Responder};
use std::{fs, io::Write, path::PathBuf};
use uuid::Uuid;

use crate::states::{DB, FILES};

#[get("/upload/{file_url}")]
async fn upload_from_url(db: web::Data<DB>, file_url: web::Path<String>) -> impl Responder {
    let response = reqwest::get(file_url.into_inner()).await.unwrap();
    let body = response.bytes().await.unwrap();
    let mime = infer::get(&body).unwrap();
    let id = Uuid::new_v4();

    let location = "/";
    let file_name = format!("{}.{}", id.to_string(), mime.extension());

    let path = PathBuf::from(format!("{}{}{}", FILES, location, file_name));
    let mut file_fs = fs::File::create(path).unwrap();

    file_fs.write_all(&body).unwrap();

    let file_record = content::File {
        id,
        name: file_name.to_string(),
        location: format!("{}{}", location, file_name),
        mime: mime.to_string(),
        size: body.len() as u64,
    };

    db.tables.file().save(file_record.clone()).await;

    serde_json::to_string(&file_record)
}
