use std::fmt::Write;
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use crate::models::tables::{Post, Comment};

//rust html thingee...?
//make not async?

pub fn registration_page(flash_messages: &IncomingFlashMessages) -> String
{
    let mut error_html: String = String::new();

    for message in flash_messages.iter().filter(|message| message.level() == Level::Error) 
    {
        writeln!(error_html, "<p><i>{}</i></p>", message.content()).unwrap();
    }

    format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Registration</title>
            </head>
            <body>
                {error_html}
                <form action="/register" method="post">
                    <label>Username
                        <input 
                            type="text" 
                            placeholder="Enter Username" 
                            name="username"
                        >
                    </label>
                    <label>Password
                        <input 
                            type="password" 
                            placeholder="Enter Password"
                            name="password"
                        >
                    </label>
                    <label>Email (optional)
                        <input
                            type="text"
                            placeholder="Enter Email"
                            name="email"
                        >
                    </label>
                    <button type="submit">Register</button>
                </form>
            </body>
        </html>
        "#
    )
}


pub fn login_page(flash_messages: &IncomingFlashMessages) -> String
{
    let mut error_html: String = String::new();

    for message in flash_messages.iter().filter(|message| message.level() == Level::Error) 
    {
        writeln!(error_html, "<p><i>{}</i></p>", message.content()).unwrap();
    }

    format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Login</title>
            </head>
            <body>
                {error_html}
                <form action="/login" method="post">
                    <label>Username
                        <input 
                            type="text" 
                            placeholder="Enter Username" 
                            name="username"
                        >
                    </label>
                    <label>Password
                        <input 
                            type="password" 
                            placeholder="Enter Password"
                            name="password"
                        >
                    </label>
                    <button type="submit">Login</button>
                </form>
            </body>
        </html>
        "#
    )
}


pub fn landing_page(posts: &Vec<Post>) -> String
{
    let mut post_list: String = String::new();

    for post in posts.iter()
    {
        writeln!(
                post_list,
                "<div>
                    <a href='/p/{}'>{}</a>
                    <p>{}  {} id:{}</p>
                </div>",
                post.id, post.title, post.user, post.timestamp, post.id
            )
             .unwrap();
    }

    format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta http-equiv="content-type" content="text/html; charset=utf-8">
                    <title>Welcome to Phooey</title>
                    <link rel="shortcut icon" type="image/x-icon" href="/favicon.ico?v=1.0">
                </head>
                <h1>Welcome motherfucker!</h1>
                <body>
                    <div>
                        <a href='/register'>Register</a>
                    </div>
                    <div>
                        <a href='/login'>Login</a>
                    </div>
                    <p></p>
                    <p></p>
                    {post_list}
                </body>
            </html>
            "#
    )   
}



pub fn home_page(username: &String, posts: &Vec<Post>) -> String
{
    let mut post_list: String = String::new();

    for post in posts.iter()
    {
        writeln!(
            post_list,
            "<div>
                <a href='/p/{}'>{}</a>
                <p>{}  {} id:{}</p>
            </div>",
            post.id, post.title, post.user, post.timestamp, post.id
        )
         .unwrap();
    }

    format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Phooey Home</title>
            </head>
            <body>
                <h1>Welcome {username}!</h1>
                <a href='/post'>Click here to get posty</a>
                <p></p>
                <p></p>
                {post_list}
            </body>
        </html>
        "#
    )
}


pub fn post_view_page(post: &Post, comments: &Vec<Comment>) -> String
{
    let mut comment_html: String = String::new();
    let mut css: String = String::new();

    let mut number:i32 = 1;

    for comment in comments.iter()
    {
        let fart: String = format!(
            r#"
            <label for="trigger{number}">Reply</label>
            <input id="trigger{number}" type="checkbox" class="chk-btn{number}">
            <div class="box{number}">
                <form action="/p/{}/{}" method="post">
                    <textarea id="body" name="body" rows="10" cols="50"></textarea>
                    <button type="submit">Post</button>
                </form>
            </div>
            "#,
            comment.post_id, comment.id
        );
 

        let delim = ".";
        let depth: i32 = comment.path_string.matches(delim).count().try_into().unwrap();
        let margin: i32 = 40*depth; 


        let thing: String = format!(
            r#"<div style="margin-left: {margin}px;">
                    <p>{}</p>
                    <p></p>
                    <p>By {}</p>
                    <p>id {}</p>
                </div>
                {fart}
                "#,
                comment.body, comment.username, comment.id
        );

        writeln!(comment_html, "{thing}").unwrap();

        let dill: String = format!(
            r#".box{number} 
            {{
                display: none;
                background: #ccc;
                width: 200px;
                height: 200px;
            }}
            
            #trigger{number}:checked + .box{number} 
            {{
                display: block;
            }}
        
            input.chk-btn{number} 
            {{
                display: none;
            }}
               
            input.chk-btn{number} + label{number} 
            {{
                border: 1px solid grey;
                background: ghoswhite;
                padding: 5px 8px;
                cursor: pointer;
                border-radius: 5px;
            }}"#
            );

        writeln!(css, "{}", dill).unwrap();
        number +=1;
    }

    format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <style>
                {css}
                </style>
            </head>
            <body>
                <h1>{}</h1>
                <div>{}</div>
                <p>By {} @{}</p>
                <br>
                <div>
                    <form action="/p/{}" method="post">
                        <label>Comment
                            <br>
                            <textarea id="body" name="body" rows="10" cols="50"></textarea>
                        </label>
                        <button type="submit">Post</button>
                    </form>
                </div>
                <div>{comment_html}</div>
            </body>
        </html>
        "#,
        post.title, post.body, post.user, post.timestamp.to_string(), post.id
    )
}


pub fn submit_form() -> String
{
    format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title> Submit Post</title>
            </head>
            <body>
                <form action="/post" method="post">
                    <div>
                        <label>Title</label>
                            <br>
                            <input
                                type="text"
                                placeholder="Enter Title"
                                name="title"
                            >
                    </div>
                    <div>
                        <label>Text
                            <br>
                            <textarea id="body" name="body" rows="10" cols="50"></textarea>
                        </label>
                    </div>
                    <div>
                        <button type="submit">Post</button>
                    </div>
                </form>
            </body>
        "#
    )
}