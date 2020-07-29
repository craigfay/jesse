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

pub fn create_post(new_post: &models::PostInsertion) {
    let connection = establish_connection();

    diesel::insert_into(schema::posts::table)
        .values(new_post)
        .execute(&connection);
}

pub fn mutate_post(id: i32, mutation: &models::PostMutation) {
    let connection = establish_connection();
    diesel::update(
        schema::posts::table.filter(schema::posts::id.eq(id))
    )
        .set(mutation)
        .execute(&connection);
}

pub fn read_post(id: i32) -> Option<models::Post> {
    let connection = establish_connection();
    let result = schema::posts::table
        .find(id)
        .first::<models::Post>(&connection);
    
    match result {
        Err(_e) => None,
        Ok(post) => Some(post),
    }
}

pub fn read_posts() -> Vec<models::Post> {
    let connection = establish_connection();
    schema::posts::table
        .limit(5)
        .load::<models::Post>(&connection)
        .expect("Error loading posts")
}


