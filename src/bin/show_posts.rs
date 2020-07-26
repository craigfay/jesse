extern crate diesel;
extern crate rust_json_api;

use rust_json_api::db::models::*;
use diesel::prelude::*;

fn main() {
    use rust_json_api::db::schema::posts::dsl::*;
    use rust_json_api::db::establish_connection;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
