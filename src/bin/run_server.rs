

extern crate rust_json_api;

use actix_web::{web, App, HttpResponse, HttpServer};
use rust_json_api::db;
use serde_json;
use serde::{Serialize, Deserialize};


#[derive(Deserialize)]
struct Params {
    id: i32,
}


#[derive(Serialize)]
struct RestResponse<T> {
    data: Option<T>,
    errors: Vec<String>,
}

async fn read_post_handler(params: web::Path<Params>) -> HttpResponse {
    let id = params.id;
    let post = db::read_post(id);
    let response = match post {
        None => {
            RestResponse::<db::models::Post> {
                data: None,
                errors: vec!["Not Found".to_string()],
            }
        }
        Some(post) => {
            RestResponse::<db::models::Post> {
                data: Some(post),
                errors: vec![],
            }
        }
    };

    let json = serde_json::to_string(&response).unwrap();
    HttpResponse::Ok()
      .content_type("application/json")
      .body(json)

}


async fn read_posts_handler() -> HttpResponse {
    let posts = db::read_posts();
    let response = RestResponse::<Vec<db::models::Post>> {
        data: Some(posts),
        errors: vec![],
    };
    let json = serde_json::to_string(&response).unwrap();
    HttpResponse::Ok()
      .content_type("application/json")
      .body(json)
}

async fn create_posts_handler(data: web::Json<db::models::PostInsertion>) -> HttpResponse {

    // TODO return the new record if possible
    // TODO this fails, not sure why
    let _result = db::create_post(&data);

    let response = RestResponse::<db::models::PostInsertion> {
        data: Some(data.into_inner()),
        errors: vec![],
    };

    let json = serde_json::to_string(&response).unwrap();
    HttpResponse::Ok()
      .content_type("application/json")
      .body(json)
}

async fn mutate_post_handler(
    path: web::Path<Params>,
    data: web::Json<db::models::PostMutation>
) -> HttpResponse {
    let id = path.id;
    let _result = db::mutate_post(id, &data);

    // TODO handle errors
    // TODO return better response
    let response = RestResponse::<db::models::PostMutation> {
        data: Some(data.into_inner()),
        errors: vec![],
    };

    let json = serde_json::to_string(&response).unwrap();
    HttpResponse::Ok()
      .content_type("application/json")
      .body(json)
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
                .route("/posts.json",
                    web::post().to(create_posts_handler))
                .route("/posts/{id}.json",
                    web::put().to(mutate_post_handler))
        )

    })
    .bind(&addr)?
    .run()
    .await
}




