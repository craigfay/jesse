extern crate diesel;
//extern crate diesel_demo_step_1_pg;

use rust_json_api::models::*;
use diesel::prelude::*;
//use diesel_demo_step_1_pg::*;

fn main() {
    use rust_json_api::schema::posts::dsl::*;
    use rust_json_api::establish_connection;

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
