use super::schema::posts;
use serde::{Serialize, Deserialize};
use diesel::Insertable;
use diesel::query_builder::AsChangeset;

pub trait ModelInsertion<'a, M>:
Insertable<M>
+ Serialize
+ Deserialize<'a>
+ AsChangeset {
}

pub struct Model {
    name: String,
    //insertable: dyn ModelInsertion<Model>,
}

pub trait Resource {
    type Insertable;
}


pub struct Book;

impl Resource for Book {
    type Insertable = BookInsertion;
}

pub struct BookInsertion {
    pub title: String,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Queryable)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[derive(Serialize)]
#[derive(Deserialize)]
#[table_name = "posts"]
pub struct PostInsertion {
    pub title: String,
    pub body: String,
}

#[derive(Insertable)]
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(AsChangeset)]
#[table_name = "posts"]
pub struct PostMutation {
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
}

