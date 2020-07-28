
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

extern crate rust_json_api;


use rust_json_api::db;
use serde_json;
use serde::{Serialize, Deserialize};


#[derive(Deserialize)]
struct Params {
    id: i32,
}


#[derive(Serialize)]
struct RestResponse<M> {
    data: Option<M>,
    errors: Vec<String>,
}


async fn read_post_handler(params: web::Path<Params>) -> impl Responder {
    let id = params.id;
    let post = db::read_post(id);
    match post {
        None => {
            let response = RestResponse::<db::models::Post> {
                data: None,
                errors: vec!["Not Found".to_string()],
            };
            let json = serde_json::to_string_pretty(&response).unwrap();
            format!("{}", json)
        }
        Some(post) => {
            let response = RestResponse::<db::models::Post> {
                data: Some(post),
                errors: vec![],
            };
            let json = serde_json::to_string_pretty(&response).unwrap();
            format!("{}", json)
        }
    }

}


async fn read_posts_handler() -> impl Responder {
    let posts = db::read_posts();
    let json = serde_json::to_string_pretty(&posts).unwrap();
    format!("{}", json)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8000";
    println!("Listening at {}", addr);

    HttpServer::new(|| {
        App::new().service(

            web::scope("/api")
                .route("/posts.json",
                    web::get().to(read_posts_handler))
                .route("/posts/{id}.json",
                    web::get().to(read_post_handler))
        )

    })
    .bind(&addr)?
    .run()
    .await

}




