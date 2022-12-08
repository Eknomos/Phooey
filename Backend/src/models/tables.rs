use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use chrono::{DateTime, Utc};


#[derive(Deserialize, Serialize)]
pub struct User
{
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub timestamp: DateTime<Utc>
}

impl From<Row> for User 
{
    fn from(row: Row) -> Self 
    {
        Self 
        {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password: row.get("password"),
            timestamp: row.get("timestamp")
        }
    }
}


#[derive(Deserialize, Serialize)]
pub struct Post
{
    pub id: i32,
    pub title: String,
    pub body: String,
    pub user: String,
    pub timestamp: DateTime<Utc>
}

impl From<Row> for Post
{
    fn from(row: Row) -> Self
    {
        Self
        {
            id: row.get("id"),
            title: row.get("title"),
            body: row.get("body"),
            user: row.get("username"),
            timestamp: row.get("timestamp")
        }
    }
}


#[derive(Deserialize, Serialize)]
pub struct Comment
{
    pub id: i32,
    pub post_id: i32,
    pub body: String,
    pub username: String,
    pub path_string: String
}

impl From<Row> for Comment
{
    fn from(row: Row) -> Self
    {
        Self
        {
            id: row.get("id"),
            post_id: row.get("post_id"),
            body: row.get("body"),
            username: row.get("username"),
            path_string: row.get("path_string")
        }
    }
}


