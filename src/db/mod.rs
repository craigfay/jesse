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

pub fn create<M: models::Resource>() -> bool {
    M::name() == "Book"
}

pub fn create_post(new_post: &models::PostInsertion) -> Result<usize, &'static str>{
    let connection = establish_connection();

    let result = diesel::insert_into(schema::posts::table)
        .values(new_post)
        .execute(&connection);

    match result {
        Ok(insertion_count) => Ok(insertion_count),
        Err(e) => match e {
            _ => Err("Unknown Insertion Error"),
            // TODO distinguish client / server database errors
            //diesel::result::Error::DatabaseError() => Err("Invalid Insertion Data"),
        } 
    }
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
        .load::<models::Post>(&connection)
        .expect("Error loading posts")
}


