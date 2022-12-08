use deadpool_postgres::Client;
use tokio_postgres::Row;
use crate::models::forms::{RegistrationForm, LoginForm, PostForm, CommentForm};
use crate::models::tables::{User, Post, Comment};
use crate::models::errors::PhooeyError;
use chrono::{Utc, DateTime};
use actix_session::Session;


pub async fn validate_user(session: &Session) -> Result<String, PhooeyError>
{
    let username: String = if let Some(name) = session
    .get::<String>("name")
    .map_err(|_| PhooeyError::DatabaseError)?
    {
        name.to_string()
    }
    else
    {
        return Err(PhooeyError::NotLoggedIn);
    };
    return Ok(username);
}


pub async fn insert_user(connection: &Client, form: &RegistrationForm) -> Result<u64,PhooeyError>
{
    let time: DateTime<Utc> = Utc::now();
    Ok(connection.execute(
        "INSERT INTO users (username, email, password, timestamp) 
                    VALUES ($1, $2, $3, $4)",
        &[&form.username, &form.email, &form.password, &time],
    )
    .await
    .map_err(|_| PhooeyError::InvalidData)?)
}


pub async fn get_user(connection: &Client, form: &LoginForm) -> Result<User, PhooeyError>
{
    Ok(User::from(
        connection.query_one(
            "SELECT * FROM users WHERE (username) = $1 AND (password) = $2", 
            &[&form.username, &form.password]
    )
    .await
    .map_err(|_| PhooeyError::InvalidData)?))
}


pub async fn get_users(connection: &Client) -> Result<Vec<User>, PhooeyError>
{    
    let rows: Vec<Row> = connection.query(
        "SELECT * FROM users", 
        &[]
    )
    .await
    .map_err(|_| PhooeyError::InvalidData)?;

    let users: Vec<User> = rows
            .into_iter()
            .map(|row| User::from(row) )
            .collect();

    Ok(users)   
}


pub async fn insert_post(connection: &Client, form: &PostForm, username: &String) -> Result<u64, tokio_postgres::Error>
{
    let time: DateTime<Utc> = Utc::now();

    Ok(connection.execute(
        "INSERT INTO posts (title, body, username, timestamp) 
                    VALUES ($1, $2, $3, $4)",
        &[&form.title, &form.body, &username, &time],
    )
    .await?)
}


pub async fn get_post(connection: &Client, id: &i32) -> Result<Post, PhooeyError>
{
    Ok(Post::from(
        connection.query_one(
            "SELECT * FROM posts WHERE (id) = $1", 
            &[&id]
    )
    .await
    .map_err(|_| PhooeyError::InvalidData)?))
}


pub async fn get_posts(connection: &Client) -> Result<Vec<Post>, PhooeyError>
{
    let rows: Vec<Row> = connection.query(
        "SELECT * FROM posts ORDER BY id DESC", 
        &[]
    )
    .await
    .map_err(|_| PhooeyError::InvalidData)?;

    let posts: Vec<Post> = rows
            .into_iter()
            .map(|row| Post::from(row))
            .collect();

    Ok(posts)   
}


pub async fn post_comment(connection: &Client, comment: &CommentForm, post_id: &i32, username: &String) -> Result<u64, PhooeyError>
{

    let x = connection.query_one(
        r#"
        Select cast (nextval(pg_get_serial_sequence('nipple', 'id')) as VARCHAR) as new_id;
        "#,
        &[]
    ).await
    .expect("penis");
    //.map_err(|_| PhooeyError::InvalidData)?);

    let y: String = x.get("new_id");
    let num: i32 = y.parse::<i32>().expect("bad num");

    let a: i32 = num;

    let path: String = format!("{:0>10}", a);

    Ok(connection.execute(
        r#"INSERT INTO comments (id, post_id, body, username, path, path_string)
        VALUES ($1, $2, $3, $4, $5, $6)"#,
        &[&a, &post_id, &comment.body, &username, &path, &path]
    )
    .await
    .expect("uhoh"))


}


pub async fn insert_comment(connection: &Client, comment: &CommentForm, post_id: &i32, comment_id: &i32, username: &String) -> Result<u64, PhooeyError>
{
    let parent: Comment = get_comment(&connection, &comment_id).await?;

    println!("comment id {}", comment_id);

    let parent_path: String = parent.path_string;

    println!("parent path {}", parent_path);
    
    let x = connection.query_one(
        r#"
        Select cast (nextval(pg_get_serial_sequence('nipple', 'id')) as VARCHAR) as new_id;
        "#,
        &[]
    ).await
    .expect("penis");
    //.map_err(|_| PhooeyError::InvalidData)?);

    let y: String = x.get("new_id");
    let num: i32 = y.parse::<i32>().expect("bad num");

    let a: i32 = num;

    println!("this post id {}", a);

    let leg: String = format!("{:0>10}", a);
    let fuckle: String = format!("{}.{}", parent_path, leg);

    println!("this leg {}", leg);
    println!("this path {}", fuckle);


    Ok(connection.execute(
        r#"INSERT INTO comments (id, post_id, body, username, path, path_string)
        VALUES ($1, $2, $3, $4, $5, $6)"#,
        &[&a, &post_id, &comment.body, &username, &fuckle, &fuckle]
    )
    .await
    .map_err(|_| PhooeyError::InvalidData)?)

}


pub async fn get_comments(connection: &Client, post_id: &i32) -> Result<Vec<Comment>, PhooeyError>
{
    let rows: Vec<Row> = connection.query(
        r#"SELECT * FROM comments WHERE (post_id) = $1 ORDER BY path, id"#, 
        &[&post_id]
    )
    .await
    .map_err(|_| PhooeyError::InvalidData)?;

    let comments: Vec<Comment> = rows
        .into_iter()
        .map(|row| Comment::from(row))
        .collect();

    Ok(comments)        
}


async fn get_comment(connection: &Client, comment_id: &i32) -> Result<Comment, PhooeyError>
{
    Ok(Comment::from(
        connection.query_one(
            "SELECT * FROM comments WHERE (id) = $1", 
            &[&comment_id]
    )
    .await
    .map_err(|_| PhooeyError::InvalidData)?))
}