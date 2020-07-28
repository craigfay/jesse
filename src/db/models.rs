use super::schema::posts;
use serde::{Serialize, Deserialize};


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
pub struct NewPost {
    pub title: String,
    pub body: String,
}

