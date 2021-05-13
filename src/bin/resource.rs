
use rust_json_api::db;

pub fn main() {
    let insertable = db::models::BookInsertion {
        title: "Harry Potter".to_string(),
    };

    let result = db::create::<db::models::Book>(insertable);

    println!("{:#?}", result);
}