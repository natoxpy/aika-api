use std::fs;

use actix_web::{get, http::header, web, HttpResponse, Responder};

use crate::states::FILES;

#[get("/{filename}")]
async fn get_file(path: web::Path<String>) -> impl Responder {
    let filename: String = path.into_inner();

    let mut path = std::path::PathBuf::new();

    path.push(FILES);
    path.push(filename);

    let file = fs::read(path).unwrap();

    let mut response = HttpResponse::Ok();

    response.insert_header(("Content-Type", mime::MPEG.to_string()));
    response.insert_header(header::ContentLength(file.len()));
    response.insert_header((header::TRANSFER_ENCODING, "chunked"));

    response.body(file)
}


