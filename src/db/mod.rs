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



pub fn create_post<'a>(conn: &SqliteConnection, title: &'a str, body: &'a str) {

    let new_post = models::NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(schema::posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");
}
