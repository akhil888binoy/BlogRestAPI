
use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    App, HttpResponse,
};
use derive_more::derive::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum MyError {
    #[display("internal error")]
    InternalError,

    #[display("bad request")]
    BadClientData,

    #[display("timeout")]
    Timeout,

    #[display("no data found")]
    NotFound,

    #[display("Incorrect Password")]
    IncorrectPassword
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            MyError::NotFound => StatusCode::NOT_FOUND,
            MyError::IncorrectPassword=>StatusCode::UNAUTHORIZED
        }
    }
}

