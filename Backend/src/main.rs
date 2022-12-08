use actix_web::{web, App, HttpServer, middleware::Logger, cookie::Key};
use actix_cors::Cors;
use actix_web_flash_messages::{FlashMessagesFramework, storage::CookieMessageStore};
use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use phooey::api;
use phooey::database::pool::get_db_pool;
use phooey::web::routes;
//use phooey::database::migrate::{migrate_db, init_db};
//use secrecy::ExposeSecret;


//flash no title
//api needs viewposts route

#[tokio::main]
async fn main() -> std::io::Result<()> 
{
    //init_db(get_db_pool()).await;
    //migrate_db(get_db_pool()).await;
    //Ok(());

    std::env::set_var("RUST_LOG","actix_web=info");
    emoji_logger::init();

    //not that secure, replaces with hmac
    let signing_key: Key = Key::generate();

    //gonna want to use redis session storage here
    let message_store: CookieMessageStore = CookieMessageStore::builder(signing_key.clone()).build();
    let message_framework: FlashMessagesFramework = FlashMessagesFramework::builder(message_store).build();

    let redis: RedisSessionStore = RedisSessionStore::new("redis://127.0.0.1:6379").await.unwrap();

    HttpServer::new(move || 
    {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .wrap(message_framework.clone())
            
            .wrap(SessionMiddleware::new(redis.clone(), signing_key.clone()))

            .route("/", web::get().to(routes::landing))
            .route("/favicon.ico", web::get().to(routes::favicon))
            .route("/register", web::get().to(routes::registration_page))
            .route("/register", web::post().to(routes::register))
            .route("/login", web::get().to(routes::login_page))
            .route("/login", web::post().to(routes::login))
            .route("/home", web::get().to(routes::home))            
            .route("/post", web::get().to(routes::submit_post_page))
            .route("/post", web::post().to(routes::submit_post))
            .route("/p/{post_id}", web::get().to(routes::view_post))
            .route("/p/{post_id}", web::post().to(routes::submit_comment))            
            .route("/p/{post_id}/{comment_id}", web::post().to(routes::comment_reply))

            .route("/api", web::get().to(api::routes::landing))
            .route("/api/register", web::post().to(api::routes::register))
            .route("/api/login", web::post().to(api::routes::login))
            .route("/api/post", web::post().to(api::routes::submit_post))
            .route("/api/{post_id}", web::get().to(api::routes::view_post))
            
            .app_data(web::Data::new(get_db_pool()))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
