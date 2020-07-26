extern crate diesel;
extern crate rust_json_api;

use rust_json_api::db::models::*;
use diesel::prelude::*;
use serde_json;

fn main() {
    use rust_json_api::db::schema::posts::dsl::*;
    use rust_json_api::db::establish_connection;

    let connection = establish_connection();
    let results = posts
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");


    let json = serde_json::to_string_pretty(&results).unwrap();
    println!("{}", json);
}
