
use rust_json_api::db;

pub fn main() {
    let result = db::create::<db::models::Book>();
    println!("{:#?}", result);
}