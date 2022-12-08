use derive_more::{Display, From};
use actix_web::{HttpResponse, ResponseError};
use actix_web::http::header::{LOCATION};

#[derive(Display, From, Debug)]
pub enum PhooeyError 
{
    DatabaseError,
    InvalidData,
    NotLoggedIn
}

impl std::error::Error for PhooeyError {}

impl ResponseError for PhooeyError 
{
    fn error_response(&self) -> HttpResponse 
    {
        match *self 
        {
            PhooeyError::DatabaseError => HttpResponse::InternalServerError().finish(),
            PhooeyError::InvalidData => HttpResponse::BadRequest().finish(),
            PhooeyError::NotLoggedIn => HttpResponse::SeeOther().insert_header((LOCATION, "/login")).finish()
        }
    }
}