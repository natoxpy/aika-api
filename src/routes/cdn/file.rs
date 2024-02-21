use std::{fs, path::PathBuf};

use actix_web::{get, http::header, web, HttpResponse, Responder};

use crate::states::{DB, FILES};

#[get("/{file_id}")]
async fn file_cdn(db: web::Data<DB>, path: web::Path<String>) -> impl Responder {
    let file_id: String = path.into_inner();

    let file_opt = db.tables.files().get(file_id.clone()).await;

    if file_opt.is_none() {
        return HttpResponse::NoContent().into();
    }

    let file_db = file_opt.unwrap();

    let path = PathBuf::from(format!("{}{}", FILES, file_db.location));

    let file = fs::read(path).unwrap();

    let mut response = HttpResponse::Ok();

    let mime: mime::Mime = file_db.mime.parse().unwrap();

    response.insert_header(("Content-Type", mime.to_string()));
    response.insert_header(header::ContentLength(file.len()));
    response.insert_header((header::TRANSFER_ENCODING, "chunked"));

    response.body(file)
}
