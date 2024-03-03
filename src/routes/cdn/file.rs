use std::{fs, path::PathBuf};

use actix_web::{get, http::header, web, HttpResponse};

use crate::states::{DB, FILES};

#[get("/{file_id}")]
async fn file_cdn(
    db: web::Data<DB>,
    path: web::Path<String>,
) -> Result<HttpResponse, crate::routes::Error> {
    let file_id: String = path.into_inner();

    let file_record = db
        .tables
        .files()
        .get(file_id.clone())
        .await
        .map_err(|err| crate::routes::Error::DB(err))?;

    let path = PathBuf::from(format!("{}{}", FILES, file_record.location));
    let file = fs::read(path).map_err(|err| crate::routes::Error::IO(err))?;

    let mut response = HttpResponse::Ok();

    let mime = file_record
        .mime
        .parse::<mime::Mime>()
        .map_err(|err| crate::routes::Error::FromStr(err))?;

    response.insert_header(("Content-Type", mime.to_string()));
    response.insert_header(header::ContentLength(file.len()));
    response.insert_header((header::TRANSFER_ENCODING, "chunked"));

    Ok(response.body(file))
}
