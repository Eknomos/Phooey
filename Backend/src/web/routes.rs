use actix_web::{web, HttpResponse};
use actix_web::http::header::{ContentType, LOCATION};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use actix_files::NamedFile;
use actix_session::Session;
use deadpool_postgres::{Client, Pool};
use crate::models::forms::{RegistrationForm, LoginForm, PostForm, CommentForm};
use crate::models::tables::{Post, Comment};
use crate::models::errors::PhooeyError;
use crate::web::{pages, utils::redirect, utils::send_page};
use crate::database::services::{get_user, insert_user, get_posts, get_post, insert_post, get_comments, post_comment, validate_user, insert_comment};


pub async fn landing(pool: web::Data<Pool>,) -> Result<HttpResponse, PhooeyError>
{
    let connection: Client = pool.get().await.map_err(|_| PhooeyError::DatabaseError)?;
    let posts: Vec<Post> = get_posts(&connection).await.unwrap();
    let html: String = pages::landing_page(&posts);
    return Ok(send_page(html));
}


pub async fn favicon() -> actix_web::Result<NamedFile, PhooeyError>
{
    Ok(NamedFile::open("/Users/natedawg/Phooey/Backend/src/web/assets/favicon.ico").map_err(|_| PhooeyError::DatabaseError)?)
}


pub async fn login_page(flash_messages: IncomingFlashMessages) -> HttpResponse
{
    let html: String = pages::login_page(&flash_messages);
    return send_page(html);
}


pub async fn login(form: web::Form<LoginForm>, pool: web::Data<Pool>, session: Session) -> Result<HttpResponse, PhooeyError>
{
    let connection: Client = pool.get().await.map_err(|_| PhooeyError::DatabaseError)?;
    
    match get_user(&connection, &form).await
    {
        Ok(user) => 
        {
            session.renew();
            session.insert("id", user.id).map_err(|_| PhooeyError::DatabaseError)?;
            session.insert("name", user.username).map_err(|_| PhooeyError::DatabaseError)?;
            return Ok(redirect("/home"));
        }
        Err(_) => 
        {
            FlashMessage::error("Bad credentials nibba!").send();
            return Ok(redirect("/login"));
        }
    }
}


pub async fn register(form: web::Form<RegistrationForm>, pool: web::Data<Pool>) -> Result<HttpResponse, PhooeyError>
{
    let connection: Client = pool.get().await.map_err(|_| PhooeyError::DatabaseError)?;

    match insert_user(&connection, &form).await
    {
        Ok(_) => 
        {
            return Ok(redirect("/login"));
        }
        Err(_) => 
        {
            FlashMessage::error("Can't register with those credentials nibba").send();
            return Ok(redirect("/register"));
        }
    }
}


pub async fn registration_page(flash_messages: IncomingFlashMessages) -> HttpResponse
{
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(pages::registration_page(&flash_messages))
}


pub async fn home(session: Session, pool: web::Data<Pool>) -> Result<HttpResponse, PhooeyError>
{

    let username: String = validate_user(&session).await?;

        let connection: Client = pool.get().await.map_err(|_| PhooeyError::DatabaseError)?;
        let posts: Vec<Post> = get_posts(&connection).await?;

        Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(pages::home_page(&username, &posts))
        )
}


pub async fn view_post(path: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse, PhooeyError>
{
    let connection: Client = pool.get().await.map_err(|_| PhooeyError::DatabaseError)?;
    let post_id: i32 = path.into_inner().parse().unwrap();
    let post: Post = get_post(&connection, &post_id).await?;
    let comments: Vec<Comment> = get_comments(&connection, &post_id).await?;

    println!("{}", pages::post_view_page(&post, &comments));

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(pages::post_view_page(&post, &comments))
    )
}


pub async fn submit_post_page(_flash_messages: IncomingFlashMessages, session: Session) -> Result<HttpResponse, PhooeyError>
{
    validate_user(&session).await?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(pages::submit_form()))
}


pub async fn submit_post(form: web::Form<PostForm>, pool: web::Data<Pool>, session: Session) -> Result<HttpResponse, PhooeyError>
{
    let username: String = validate_user(&session).await?;
    let connection: Client = pool.get().await.map_err(|_| PhooeyError::DatabaseError)?;
    
    insert_post(&connection, &form, &username).await.unwrap();
    
    return Ok(redirect("/home"));
}


pub async fn submit_comment(form: web::Form<CommentForm>, pool: web::Data<Pool>, session: Session, path: web::Path<String>) -> Result<HttpResponse, PhooeyError>
{
    let username: String = validate_user(&session).await?;
    let connection: Client = pool.get().await.map_err(|_| PhooeyError::DatabaseError)?;
    let post_id: i32 = path.into_inner().parse().unwrap();
    let route: String = format!("/p/{}", post_id);

    post_comment(&connection, &form, &post_id, &username).await?;

    Ok(HttpResponse::SeeOther().insert_header((LOCATION, route)).finish())
}


pub async fn comment_reply(form: web::Form<CommentForm>, pool: web::Data<Pool>, session: Session, path: web::Path<(String, String)>) -> Result<HttpResponse, PhooeyError>
{
    let username: String = validate_user(&session).await?;
    let connection: Client = pool.get().await.map_err(|_| PhooeyError::DatabaseError)?;
    let (post_id, comment_id) = path.into_inner();
    let pid: i32 = post_id.parse().unwrap();
    let cid: i32 = comment_id.parse().unwrap();
    let route: String = format!("/p/{}", pid);

    insert_comment(&connection, &form, &pid, &cid, &username).await?;

    Ok(HttpResponse::SeeOther().insert_header((LOCATION, route)).finish())
}