// TODO:
// Fix the todo! hell
//
use actix_web::{HttpResponse, ResponseError};
use mime::FromStrError;
use reqwest::StatusCode;
use sclouddl::error::ScloudError;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    DB(crate::db::Error),
    FromStr(FromStrError),
    ParseUrl(url::ParseError),
    IO(std::io::Error),
    Reqwest(reqwest::Error),
    Other(String),
    Scloud(ScloudError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DB(error) => write!(f, "{:?}", error),
            Error::FromStr(error) => write!(f, "{:?}", error),
            Error::IO(error) => write!(f, "{:?}", error),
            Error::Reqwest(error) => write!(f, "{:?}", error),
            Error::Other(msg) => write!(f, "{}", msg),
            Error::Scloud(error) => write!(f, "{:?}", error),
            Error::ParseUrl(error) => write!(f, "{:?}", error),
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).body(format!("{}", self))
    }

    fn status_code(&self) -> reqwest::StatusCode {
        let status_code = match self {
            Error::DB(error) => match error {
                crate::db::Error::Sqlx(sqlx_error) => match sqlx_error {
                    sqlx::Error::Configuration(_) => todo!(),
                    sqlx::Error::Database(_) => todo!(),
                    sqlx::Error::Io(_) => todo!(),
                    sqlx::Error::Tls(_) => todo!(),
                    sqlx::Error::Protocol(_) => todo!(),
                    sqlx::Error::RowNotFound => StatusCode::BAD_REQUEST,
                    #[allow(unused_variables)]
                    sqlx::Error::TypeNotFound { type_name } => todo!(),
                    #[allow(unused_variables)]
                    sqlx::Error::ColumnIndexOutOfBounds { index, len } => todo!(),
                    sqlx::Error::ColumnNotFound(_) => todo!(),
                    #[allow(unused_variables)]
                    sqlx::Error::ColumnDecode { index, source } => todo!(),
                    sqlx::Error::Decode(_) => todo!(),
                    sqlx::Error::AnyDriverError(_) => todo!(),
                    sqlx::Error::PoolTimedOut => todo!(),
                    sqlx::Error::PoolClosed => todo!(),
                    sqlx::Error::WorkerCrashed => todo!(),
                    sqlx::Error::Migrate(_) => todo!(),
                    _ => todo!(),
                },
                crate::db::Error::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Error::FromStr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::IO(_) => StatusCode::NOT_FOUND,
            Error::Reqwest(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Scloud(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ParseUrl(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        status_code
    }
}
