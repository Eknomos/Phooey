use actix_web::{HttpResponse, http::header::{LOCATION, ContentType}};


pub fn redirect(location: &str) -> HttpResponse
{
    return HttpResponse::SeeOther().insert_header((LOCATION, location)).finish();
}


pub fn send_page(html: String) -> HttpResponse
{
    return HttpResponse::Ok().content_type(ContentType::html()).body(html)
}
