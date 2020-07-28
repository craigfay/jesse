pub mod models;
pub mod schema;

use diesel::{prelude::*, sqlite::SqliteConnection};
use dotenv::dotenv;
use std::env;


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let db = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    SqliteConnection::establish(&db)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db))
}


pub fn create_post<'a>(title: &'a str, body: &'a str) {
    let connection = establish_connection();

    let new_post = models::NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(schema::posts::table)
        .values(&new_post)
        .execute(&connection)
        .expect("Error saving new post");
}

pub fn read_post(id: i32) -> models::Post {
    let connection = establish_connection();
    schema::posts::table
        .find(id)
        .first::<models::Post>(&connection)
        .expect("Error loading post")
}

pub fn read_posts() -> Vec<models::Post> {
    let connection = establish_connection();
    schema::posts::table
        .limit(5)
        .load::<models::Post>(&connection)
        .expect("Error loading posts")
}

