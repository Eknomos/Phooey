use actix_web::{web, HttpRequest, HttpResponse};
use deadpool_postgres::{Client, Pool};
use crate::models::forms::{RegistrationForm, LoginForm, PostForm};
use crate::models::tables::{User, Post};
use crate::models::errors::PhooeyError;
use crate::database::services::{insert_user, get_user, get_post};


pub async fn landing(_req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse, PhooeyError>
{
    let _connection: Client = pool.get().await.map_err(|_| PhooeyError::DatabaseError)?;
    Ok(HttpResponse::Ok().finish())
}


pub async fn register(form: web::Json<RegistrationForm>, pool: web::Data<Pool>) -> Result<HttpResponse, PhooeyError>
{
    let connection: Client = pool.get().await.map_err(|_| PhooeyError::DatabaseError)?;
    insert_user(&connection, &form).await?;
    Ok(HttpResponse::Ok().finish())
}


// add last login time?
pub async fn login(form: web::Json<LoginForm>, pool: web::Data<Pool>) -> Result<HttpResponse, PhooeyError>
{
    let connection: Client = pool.get().await.map_err(|_| PhooeyError::DatabaseError)?;
    let user: User = get_user(&connection, &form).await?;
    Ok(HttpResponse::Ok().json(web::Json(user)))
}


pub async fn submit_post(_form: web::Form<PostForm>, _pool: web::Data<Pool>) -> Result<HttpResponse, PhooeyError>
{
    //let connection: Client = pool.get().await.map_err(|_| PhooeyError::DatabaseError)?;
    //insert_post(&connection, &form).await.unwrap();
    Ok(HttpResponse::Ok().finish())
} 


//serde_json faster?
pub async fn view_post(path: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse, PhooeyError>
{
    let connection: Client = pool.get().await.map_err(|_| PhooeyError::DatabaseError)?;
    let post_id: i32 = path.into_inner().parse().unwrap();
    let post: Post = get_post(&connection, &post_id).await?;
    Ok(HttpResponse::Ok().json(web::Json(post)))
}