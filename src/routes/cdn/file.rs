use std::{fs, path::PathBuf};

use actix_web::{get, http::header, web, HttpResponse, Responder};

use crate::states::{DBState, FILES};

#[get("/{file_id}")]
async fn file_cdn(db: web::Data<DBState>, path: web::Path<String>) -> impl Responder {
    let file_id: String = path.into_inner();

    let file_opt = db.file_table.get(file_id.clone()).await;

    if file_opt.is_none() {
        return HttpResponse::NoContent().into();
    }

    let file = file_opt.unwrap();

    let path = PathBuf::from(format!("{}{}", FILES, file.location));

    let file = fs::read(path).unwrap();

    let mut response = HttpResponse::Ok();

    response.insert_header(("Content-Type", mime::MPEG.to_string()));
    response.insert_header(header::ContentLength(file.len()));
    response.insert_header((header::TRANSFER_ENCODING, "chunked"));

    response.body(file)
}
