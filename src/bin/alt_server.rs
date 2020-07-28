
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

extern crate rust_json_api;


use rust_json_api::db;
use serde_json;
use serde::{Serialize, Deserialize};


#[derive(Deserialize)]
struct Params {
    id: i32,
}


async fn read_post_handler(params: web::Path<Params>) -> impl Responder {
    let id = params.id;
    format!("Hello {}!", id)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8000";
    println!("Listening at {}", addr);

    HttpServer::new(|| {
        App::new().service(

            web::scope("/api")
                .route("/posts/{id}.json",
                    web::get().to(read_post_handler))
        )

    })
    .bind(&addr)?
    .run()
    .await

}




