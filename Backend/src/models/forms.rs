use serde::Deserialize;


#[derive(Deserialize)]
pub struct RegistrationForm 
{
    pub username: String,
    pub email: String,
    pub password: String,
}


#[derive(Deserialize)]
pub struct LoginForm
{
    pub username: String,
    pub password: String,
}


#[derive(Deserialize)]
pub struct PostForm
{
    pub title: String,
    pub body: String
}


#[derive(Deserialize)]
pub struct CommentForm
{
    pub body: String
}