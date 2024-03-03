use crate::{
    db::content::{self, File},
    routes,
};
use actix_web::{post, web, HttpResponse};
use std::{fs, io::Write, path::PathBuf};
use uuid::Uuid;

use crate::states::{DB, FILES};

pub async fn get_bytes_from_url(file_url: String) -> Result<Vec<u8>, crate::routes::Error> {
    let response = reqwest::get(file_url)
        .await
        .map_err(|err| routes::Error::Reqwest(err))?;

    response
        .bytes()
        .await
        .map_err(|err| routes::Error::Reqwest(err))
        .map(|val| val.to_vec())
}

pub async fn upload_from_bytes(bytes: Vec<u8>) -> Result<File, routes::Error> {
    let mime = infer::get(&bytes).ok_or(routes::Error::Other(String::from("Inger get bytes")))?;
    let id = Uuid::new_v4();

    let location = "/";
    let file_name = format!("{}.{}", id.to_string(), mime.extension());

    let path = PathBuf::from(format!("{}{}{}", FILES, location, file_name));
    let mut file = fs::File::create(path).map_err(|err| routes::Error::IO(err))?;

    file.write_all(&bytes)
        .map_err(|err| routes::Error::IO(err))?;

    Ok(content::File {
        id,
        name: file_name.to_string(),
        location: format!("{}{}", location, file_name),
        mime: mime.to_string(),
        size: bytes.len() as u64,
    })
}

#[post("/upload")]
async fn upload_from_buffer(
    db: web::Data<DB>,
    data: web::Bytes,
) -> Result<HttpResponse, routes::Error> {
    let file = upload_from_bytes(data.to_vec()).await?;
    db.tables
        .files()
        .save(file.clone())
        .await
        .map_err(|err| routes::Error::DB(err))?;

    Ok(HttpResponse::Ok().json(&file))
}

#[post("/upload/{file_url}")]
async fn upload_from_url(
    db: web::Data<DB>,
    file_url: web::Path<String>,
) -> Result<HttpResponse, routes::Error> {
    let file_record = upload_from_bytes(get_bytes_from_url(file_url.into_inner()).await?).await?;

    db.tables
        .files()
        .save(file_record.clone())
        .await
        .map_err(|err| routes::Error::DB(err))?;

    Ok(HttpResponse::Ok().json(file_record))
}
