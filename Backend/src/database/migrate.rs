use deadpool_postgres::{Client, Pool};


pub async fn init_db(pool: Pool)
{
    let connection: Client = pool.get().await.expect("Can't connect to database");
    create_extensions(&connection).await;

    create_users(&connection).await;
    create_posts(&connection).await;
    create_comments(&connection).await;
}


pub async fn migrate_db(pool: Pool) 
{
    let connection: Client = pool.get().await.expect("Can't connect to database");
    
    migrate_users(&connection).await;
    migrate_posts(&connection).await;
    migrate_comments(&connection).await;
}


async fn migrate_users(connection: &Client)
{
    drop_users(&connection).await;
    create_users(&connection).await;
}


async fn migrate_posts(connection: &Client)
{
    drop_posts(&connection).await;
    create_posts(&connection).await;
}


async fn migrate_comments(connection: &Client)
{
    drop_comments(&connection).await;
    create_comments(&connection).await;
}


async fn create_extensions(connection: &Client) -> u64
{
    connection.execute(
        r#"create extension if not exists ltree;
        "#, 
        &[],
    )
    .await.expect("Failed to create ltree extension")
}


async fn create_users(connection: &Client) -> u64
{
    connection.execute(
        r#"
        CREATE TABLE public.users (
            "id" SERIAL PRIMARY KEY,
            "username" VARCHAR UNIQUE,
            "email" VARCHAR,
            "password" VARCHAR NOT NULL,
            "timestamp" timestamptz NOT NULL
        );
        "#,
        &[],
    )
    .await.expect("Failed to create table 'Users'")
}


async fn drop_users(connection: &Client) -> u64
{
    connection.execute(
        r#"
        DROP TABLE public.users;
        "#,
        &[],
    )
    .await
    .expect("Failed to drop table 'Users'")
}


async fn create_posts(connection: &Client) -> u64
{
    connection.execute(
        r#"
        CREATE TABLE public.posts (
            "id" SERIAL PRIMARY KEY,
            "title" VARCHAR NOT NULL,
            "body" VARCHAR NOT NULL,
            "username" VARCHAR,
            "timestamp" timestamptz NOT NULL
        );
        "#,
        &[],
    )
    .await.expect("Failed to create table 'Posts'")
}


async fn drop_posts(connection: &Client) -> u64
{
    connection.execute(
        r#"
        DROP TABLE public.posts;
        "#,
        &[],
    )
    .await
    .expect("Failed to drop table 'Posts'")
}


async fn drop_comments(connection: &Client) -> u64
{
    connection.execute(
        r#"
        DROP TABLE public.comments;
        "#,
        &[],
    )
    .await
    .expect("Failed to drop table 'Comments'");

    connection.execute(
        r#"
        DROP TABLE public.nipple;
        "#,
        &[],
    )
    .await
    .expect("Failed to drop table 'nipple'")    

}


async fn create_comments(connection: &Client) -> u64
{
    

    connection.execute(
        r#"
        CREATE TABLE public.comments (
            "id" integer,
            "post_id" integer,
            "body" VARCHAR,
            "username" VARCHAR,
            "path" ltree,
            "path_string" VARCHAR
        );
        "#,
        &[]
    )
    .await
    .expect("Failed to create table 'Comments'");


    connection.execute(
        r#"
        CREATE TABLE public.nipple (
            "id" SERIAL
        );
        "#,
        &[]
    )
    .await
    .expect("Failed to create table nipple")

    // connection.execute(
    //     r#"
    //     CREATE INDEX path_gist_comments_idx ON comments USING GIST(path);
    //     "#,
    //     &[]
    // )
    // .await
    // .expect("Failed to create GIST index");

    // connection.execute(
    //     r#"
    //     CREATE INDEX path_comments_idx ON comments USING btree(path);        "#,
    //     &[]
    // )
    // .await
    // .expect("Failed to create BTREE index")

}

